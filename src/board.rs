use crate::{
    area::{Area, MineCount},
    position::Position,
    tile::{State, Tile, Value},
};

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
        let mut board = Self {
            tiles: vec![Tile::default(); size],
            width: config.width,
        };

        // Add mines and number tiles based on mine positions.
        mine_idxs.iter().for_each(|idx| {
            board.tiles[idx] = Tile::with_value(Value::Mine);
            // Increment number of all non-mine neighbors.
            Position::from_index(idx, config.width)
                .neighbors(config.width, config.height)
                .for_each(|pos| {
                    // Unwrap as these positions are directly from enumeration.
                    board.get_tile_mut(pos).unwrap().increment_value();
                });
        });

        board
    }

    /// Generates a boad with empty tiles at the given position, using generation config.
    #[allow(dead_code)]
    pub fn with_empty_at(_config: &GenerationConfig, _pos: Position) -> Self {
        todo!()
    }

    fn empty_area(&self, pos: Position) -> Vec<Position> {
        let mut stack = Vec::new();
        let mut emptys = Vec::new();
        let mut processed = vec![false; self.width * self.height()];

        stack.push(pos);
        while let Some(p) = stack.pop() {
            processed[p.to_index(self.width)] = true;
            emptys.push(p);

            stack.extend(p.neighbors(self.width, self.height()).filter(|p| {
                let i = p.to_index(self.width);
                !processed[i]
                    && self.tiles[i].value() == Value::Near(0)
                    && self.tiles[i].state() == State::Closed
            }));
        }

        emptys
    }

    pub fn open_from(&mut self, pos: Position) {
        if let Some(tile) = self.get_tile_mut(pos) {
            tile.open();
        }
        if let Some(tile) = self.get_tile(pos) {
            if tile.value() == Value::Near(0) {
                for p in self.empty_area(pos) {
                    self.tiles[p.to_index(self.width)].open();
                    p.neighbors(self.width, self.height())
                        .for_each(|p| self.tiles[p.to_index(self.width)].open());
                }
            }
        }
    }

    /// Opens single tile if the given position is within board bounds and
    /// tile is valid as openable i.e. it is closed.
    #[inline]
    fn open_tile(&mut self, pos: Position) {
        if let Some(tile) = self.get_tile_mut(pos) {
            tile.open();
        }
    }

    #[inline]
    pub fn flag_from(&mut self, pos: Position) {
        if let Some(tile) = self.get_tile_mut(pos) {
            tile.toggle_flag();
        }
    }

    /// Returns tile's closed neighbor tiles as [`Area`] with mine count calculated from
    /// the tile's [`Value`]. If mine count is not possible to calculate, (e.g. position is
    /// out of bounds or tile at position is a mine) returns mine count as `0..=8`.
    ///
    /// TODO: Add example.
    fn tile_neighbors_area(&self, pos: Position) -> Area {
        let flags_around = self
            .neighbors_tile_and_pos(pos)
            .filter(|(_, tile)| tile.state() == State::Flag)
            .count();

        Area::new(
            self.neighbors_tile_and_pos(pos)
                .filter_map(|(p, tile)| tile.state().eq(&State::Closed).then(|| p))
                .collect(),
            self.get_tile(pos)
                .and_then(|tile| {
                    Some(match tile.value() {
                        Value::Near(val) => MineCount::from(val as usize - flags_around),
                        Value::Mine => MineCount::from(0..=8),
                    })
                })
                .unwrap_or(MineCount::from(0..=8)),
        )
    }

    pub fn neighbors_tile_and_pos(&self, pos: Position) -> impl Iterator<Item = (Position, &Tile)> {
        pos.neighbors(self.width, self.height())
            .map(|p| (p, self.get_tile(p).unwrap()))
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
