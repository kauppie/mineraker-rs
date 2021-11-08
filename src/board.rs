use std::{collections::HashSet, ops::RangeInclusive};

use crate::tile::{State, Tile, Value};

/// [`Position`] stores 2-dimensional non-negative coordinates in uniform grid space,
/// or xy-coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
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

    pub fn neighbors(self, width: usize, height: usize) -> impl Iterator<Item = Self> {
        let (x, y) = (self.x, self.y);
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
}

pub enum MineCount {
    Defined(usize),
    RangeInclusive(RangeInclusive<usize>),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Area {
    positions: HashSet<Position>,
    // Stores the number of mines area contains.
    // Option is `None` if area contains unknown number of mines.
    mine_count: Option<usize>,
}

impl Area {
    pub fn new(positions: HashSet<Position>, mine_count: usize) -> Self {
        Self {
            positions,
            mine_count: Some(mine_count),
        }
    }

    #[inline]
    pub fn has_subarea(&self, other: &Self) -> bool {
        self.positions.is_superset(&other.positions)
    }

    pub fn difference(&self, other: &Self) -> Self {
        let other_contains = other.has_subarea(self);
        let self_contains = self.has_subarea(other);

        match (other_contains, self_contains) {
            // Areas are equivalent.
            (true, true) => Self {
                positions: self.positions.clone(),
                mine_count: self.mine_count.or(other.mine_count),
            },
            // Other contains self area.
            (true, false) => Self {
                positions: HashSet::new(),
                mine_count: Some(0),
            },
            // Self contains other area.
            (false, true) => Self {
                positions: self
                    .positions
                    .difference(&other.positions)
                    .cloned()
                    .collect(),
                mine_count: self
                    .mine_count
                    .zip(other.mine_count)
                    .and_then(|(self_mines, other_mines)| Some(self_mines - other_mines)),
            },
            // Areas may overlap.
            (false, false) => {
                let diff = self
                    .positions
                    .difference(&other.positions)
                    .cloned()
                    .collect::<HashSet<_>>();
                // Mine count can be determined only if difference area contains as many positions
                // as is the difference in the mine count between areas.
                // Difference area always contains `self_mines - other_mines..=positions.len()` mines.
                let mine_count =
                    self.mine_count
                        .zip(other.mine_count)
                        .and_then(|(self_mines, other_mines)| {
                            (diff.len() == self_mines - other_mines).then(|| diff.len())
                        });
                Self {
                    positions: diff,
                    mine_count,
                }
            }
        }
    }

    // pub fn difference2(&self, other: &Self) -> (HashSet<Position>, RangeInclusive<usize>) {
    //     let diff = self
    //         .positions
    //         .difference(&other.positions)
    //         .cloned()
    //         .collect::<HashSet<_>>();

    //     let mine_count = self
    //         .mine_count
    //         .zip(other.mine_count)
    //         .and_then(|(self_mines, other_mines)| Some(self_mines - other_mines..=diff.len()));

    //     (diff, mine_count.unwrap())
    // }
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

    pub fn flag_from(&mut self, pos: Position) {
        if let Some(tile) = self.get_tile_mut(pos) {
            tile.toggle_flag();
        }
    }

    /// Returns tile's not opened neighbor tiles as [`Area`].
    fn tile_neighbors_area(&self, pos: Position) -> Area {
        let flags_around = self
            .neighbors_tile_and_pos(pos)
            .filter(|(_, tile)| tile.state() == State::Flag)
            .count();
        Area {
            positions: self
                .neighbors_tile_and_pos(pos)
                .filter(|(_, tile)| tile.state() == State::Closed)
                .map(|(p, _)| p)
                .collect(),
            mine_count: self.get_tile(pos).and_then(|tile| match tile.value() {
                Value::Near(val) => Some(val as usize - flags_around),
                Value::Mine => None,
            }),
        }
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
