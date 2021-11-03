#[derive(Debug, Clone)]
pub enum Tile {
    // Tells the number of mines near the tile.
    Near(u8),
    Mine,
}

impl Tile {
    #[allow(dead_code)]
    pub fn is_mine(&self) -> bool {
        match self {
            Tile::Mine => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Near(0) => "_".to_string(),
                Tile::Near(val) => val.to_string(),
                Tile::Mine => "#".to_string(),
            }
        )
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Near(0)
    }
}

/// [`Position`] stores 2-dimensional non-negative coordinates in uniform grid space,
/// or xy-coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub trait BoardGenSeeder {
    fn to_u128(&self) -> u128;
    fn from_u128(seed: u128) -> Self;
    //fn from_str(bytes: &str) -> Self;
}

/// [`BoardSeed`] is a seed used for stable generation of a board.
pub struct BoardSeed(u128);

impl BoardGenSeeder for BoardSeed {
    fn to_u128(&self) -> u128 {
        self.0
    }

    fn from_u128(seed: u128) -> Self {
        BoardSeed(seed)
    }
}

/// [`GenerationConfig`] contains parameters for generating a [`Board`], including [`Seed`].
/// Two boards with same config are exactly the same in content.
pub struct GenerationConfig {
    pub seed: BoardSeed,
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    pub start_pos: Position,
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    tiles: Vec<Tile>,
    width: usize,
}

impl Board {
    /// Generates a new board with the given width, height, mine count and seed.
    ///
    /// # Panics
    /// If `mines >= width * height`.
    pub fn new(config: &GenerationConfig) -> Self {
        let size = config.width * config.height;
        assert!(config.mine_count < size, "`mines` must be less than `size`");

        // Generate mine indexes.
        // TODO: Replace with seeded rng.
        let mut rng = rand_pcg::Pcg64Mcg::new(config.seed.to_u128());
        let mine_idxs = rand::seq::index::sample(&mut rng, size, config.mine_count);

        // Setup empty board with the final size.
        let empty_board = Self {
            tiles: {
                let mut vec = Vec::new();
                vec.resize(size, Tile::Near(0));
                vec
            },
            width: config.width,
        };
        let mut numbered = empty_board.clone();
        // Add mines and number tiles based on mine positions.
        mine_idxs.iter().for_each(|idx| {
            numbered.tiles[idx] = Tile::Mine;
            // Increment number of all non-mine neighbors.
            empty_board
                .tile_neighbors(empty_board.idx_to_pos(idx))
                .for_each(|pos| {
                    // Unwrap as these coordinates are directly from enumeration and
                    // `board` and `numbered` are the same size.
                    if let Tile::Near(val) = numbered.get_tile_mut(pos).unwrap() {
                        *val += 1;
                    }
                });
        });

        numbered
    }

    /// Returns the position of the given idx given the board coordinate mapping.
    /// Position is not valid if the idx is greater than or equal to size of the board.
    #[inline]
    fn idx_to_pos(&self, idx: usize) -> Position {
        Position {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    /// Returns the index of the given position given the board coordinate mapping.
    /// Index is not be valid if it is outside the bounds of the board.
    #[inline]
    fn pos_to_idx(&self, pos: Position) -> usize {
        pos.y * self.width + pos.x
    }

    /// Returns position iterator over all board coordinates.
    #[allow(dead_code)]
    fn pos_iter(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.tiles.len()).map(move |idx| self.idx_to_pos(idx))
    }

    /// Returns iterator over tile neighbor coordinates at the given coordinates.
    /// Excludes coordinates outside the board boundaries.
    pub fn tile_neighbors(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        let (x, y) = (pos.x, pos.y);
        // Use wrapping_sub to wrap around to usize::MAX on zero values to always filter them out.
        [
            Position {
                x: x.wrapping_sub(1),
                y: y.wrapping_sub(1),
            },
            Position {
                x,
                y: y.wrapping_sub(1),
            },
            Position {
                x: x + 1,
                y: y.wrapping_sub(1),
            },
            Position {
                x: x.wrapping_sub(1),
                y,
            },
            Position { x: x + 1, y },
            Position {
                x: x.wrapping_sub(1),
                y: y + 1,
            },
            Position { x, y: y + 1 },
            Position { x: x + 1, y: y + 1 },
        ]
        .into_iter()
        .filter(|pos| pos.x < self.width && pos.y < self.height())
    }

    #[inline]
    pub fn get_tile_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        let idx = self.pos_to_idx(pos);
        self.tiles.get_mut(idx)
    }

    #[inline]
    pub fn get_tile(&self, pos: Position) -> Option<&Tile> {
        let idx = self.pos_to_idx(pos);
        self.tiles.get(idx)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.tiles.len().checked_div(self.width).unwrap_or_default()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width {
                write!(f, "{}", self.get_tile(Position { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
