use crate::tile::{Tile, Value};

/// [`Position`] stores 2-dimensional non-negative coordinates in uniform grid space,
/// or xy-coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    /// Creates a new position at given x and y coordinates.
    #[inline]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Converts index into [`Position`] in row-major order, where
    /// width is the width of each row.
    ///
    /// # Panics
    /// if `width == 0`.
    #[inline]
    pub fn from_index(index: usize, width: usize) -> Self {
        Self {
            x: index % width,
            y: index / width,
        }
    }

    /// Converts [`Position`] into index in row-major order, where
    /// width is the width of each row.
    #[inline]
    pub fn to_index(self, width: usize) -> usize {
        self.y * width + self.x
    }
}

pub trait BoardGenSeeder {
    fn to_u128(&self) -> u128;
    fn from_u128(seed: u128) -> Self;
    //fn from_str(bytes: &str) -> Self;
}

/// [`BoardSeed`] is a seed used for stable generation of a board.
/// Current version of board generation uses only the first 126 bits of the seed.
#[derive(Debug, Clone, Copy)]
pub struct BoardSeed(u128);

impl BoardGenSeeder for BoardSeed {
    fn to_u128(&self) -> u128 {
        self.0
    }

    fn from_u128(seed: u128) -> Self {
        BoardSeed(seed)
    }
}

/// [`GenerationConfig`] contains parameters for generating a [`Board`], including [`BoardSeed`].
/// Two boards with same config are exactly the same in content.
#[derive(Debug, Clone, Copy)]
pub struct GenerationConfig {
    pub seed: BoardSeed,
    // TODO: limit width and height to non-zero values.
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    // TODO: use start_pos in board generation.
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

        // Generate mine indexes using config seed.
        let mut rng = rand_pcg::Pcg64Mcg::new(config.seed.to_u128());
        let mine_idxs = rand::seq::index::sample(&mut rng, size, config.mine_count);

        // Setup empty board with the final size.
        let mut numbered = Self {
            tiles: vec![Tile::default(); size],
            width: config.width,
        };

        // Add mines and number tiles based on mine positions.
        mine_idxs.iter().for_each(|idx| {
            numbered.tiles[idx] = Tile::with_value(Value::Mine);
            // Increment number of all non-mine neighbors.
            Board::tile_neighbors_positions(
                Position::from_index(idx, config.width),
                config.width,
                config.height,
            )
            .for_each(|pos| {
                // Unwrap as these positions are directly from enumeration.
                numbered.get_tile_mut(pos).unwrap().increment_value();
            });
        });

        numbered
    }

    /// Generates a boad with empty tiles at the given position, using generation config.
    pub fn with_empty_at(config: &GenerationConfig, pos: Position) -> Self {
        todo!()
    }

    /// Returns iterator over tile's neighbors' positions.
    /// Excludes positions outside the board boundaries.
    fn tile_neighbors_positions(
        pos: Position,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = Position> {
        let (x, y) = (pos.x, pos.y);
        // Use wrapping_sub to wrap around to usize::MAX on zero values to always filter them out.
        [
            Position::new(x.wrapping_sub(1), y.wrapping_sub(1)),
            Position::new(x, y.wrapping_sub(1)),
            Position::new(x + 1, y.wrapping_sub(1)),
            Position::new(x.wrapping_sub(1), y),
            Position::new(x + 1, y),
            Position::new(x.wrapping_sub(1), y + 1),
            Position::new(x, y + 1),
            Position::new(x + 1, y + 1),
        ]
        .into_iter()
        .filter(move |pos| pos.x < width && pos.y < height)
    }

    #[inline]
    pub fn get_tile_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        let idx = pos.to_index(self.width);
        self.tiles.get_mut(idx)
    }

    #[inline]
    pub fn get_tile(&self, pos: Position) -> Option<&Tile> {
        let idx = pos.to_index(self.width);
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
