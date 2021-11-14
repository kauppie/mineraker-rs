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

pub type MineCount = RangeInclusive<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    positions: HashSet<Position>,
    // Stores the number of mines area contains.
    mine_count: MineCount,
}

impl Area {
    pub fn new(positions: HashSet<Position>, mine_count: MineCount) -> Self {
        Self {
            positions,
            mine_count,
        }
    }

    /// This constructor is used when the number of mines in given area is known to be some
    /// specific value.
    ///
    /// Following shows equivalence between this and `Area::new` function.
    /// ```
    /// use mineraker::board::Area;
    ///
    /// let area1 = Area::with_definite_mine_count(Default::default(), 5);
    /// let area2 = Area::new(Default::default(), 5..=5);
    ///
    /// assert_eq!(area1, area2);
    /// ```
    pub fn with_definite_mine_count(
        positions: HashSet<Position>,
        definite_mine_count: usize,
    ) -> Self {
        Self {
            positions,
            mine_count: definite_mine_count..=definite_mine_count,
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let intersection: HashSet<Position> = self
            .positions
            .intersection(&other.positions)
            .cloned()
            .collect();

        let self_diff = self.positions.len() - intersection.len();
        let other_diff = other.positions.len() - intersection.len();

        let min_intersection_mines = {
            let leaks_from_self = self.mine_count.start().saturating_sub(self_diff);
            let leaks_from_other = other.mine_count.start().saturating_sub(other_diff);
            let intersection_least_contains = leaks_from_self.max(leaks_from_other);

            intersection.len().min(intersection_least_contains)
        };
        let max_intersection_mines = {
            let fills_from_self = *self.mine_count.end();
            let fills_from_other = *other.mine_count.end();

            intersection
                .len()
                .min(fills_from_self)
                .min(fills_from_other)
        };

        Self {
            positions: intersection,
            mine_count: min_intersection_mines..=max_intersection_mines,
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let diff: HashSet<Position> = self
            .positions
            .difference(&other.positions)
            .cloned()
            .collect();

        let intersection_size = self.positions.intersection(&other.positions).count();

        let diff_min_mines = {
            let intersection_mines = intersection_size
                .min(*self.mine_count.start())
                .min(*other.mine_count.end());

            // This can't underflow as `intersection_mines` equal to or smaller than
            // `self.mine_count.start()` based on previous expression.
            self.mine_count.start() - intersection_mines
        };
        let diff_max_mines = {
            // Can't underflow as intersection is always equal to or smaller than
            // the area that forms it.
            let other_diff_size = other.positions.len() - intersection_size;
            let other_mines_overflow_to_intersection =
                other.mine_count.start().saturating_sub(other_diff_size);

            // Substraction can't underflow as `self.mine_count.end()` contains
            // mines that could possibly be in the intersection area and therefore
            // it is always greater or equal to mines in the intersection.
            diff.len()
                .min(self.mine_count.end() - other_mines_overflow_to_intersection)
        };

        Self {
            positions: diff,
            mine_count: diff_min_mines..=diff_max_mines,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Position;
    use crate::board::Area;
    use std::collections::HashSet;

    #[test]
    fn area_creation_equivalence() {
        let area1 = Area::new(Default::default(), 1..=1);
        let area2 = Area::with_definite_mine_count(Default::default(), 1);

        assert_eq!(area1, area2);
    }
    #[test]
    fn area_difference_with_definite_mine_count() {
        let positions1 = HashSet::from([
            Position::new(0, 1),
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
        ]);
        let positions2 = HashSet::from([
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(3, 0),
        ]);
        let diff_1_positions: HashSet<Position> =
            positions1.difference(&positions2).cloned().collect();
        let diff_2_positions: HashSet<Position> =
            positions2.difference(&positions1).cloned().collect();

        {
            let area1 = Area::with_definite_mine_count(positions1.clone(), 1);
            let area2 = Area::with_definite_mine_count(positions2.clone(), 1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::with_definite_mine_count(positions1.clone(), 2);
            let area2 = Area::with_definite_mine_count(positions2.clone(), 1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 1..=2));
        }
        {
            let area1 = Area::with_definite_mine_count(positions1.clone(), 3);
            let area2 = Area::with_definite_mine_count(positions2.clone(), 1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 2..=2));
        }
        {
            let area1 = Area::with_definite_mine_count(positions1.clone(), 1);
            let area2 = Area::with_definite_mine_count(positions2.clone(), 1);

            let diff = area2.difference(&area1);

            assert_eq!(diff, Area::new(diff_2_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::with_definite_mine_count(positions1.clone(), 2);
            let area2 = Area::with_definite_mine_count(positions2.clone(), 1);

            let diff = area2.difference(&area1);

            assert_eq!(diff, Area::new(diff_2_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::with_definite_mine_count(positions1.clone(), 3);
            let area2 = Area::with_definite_mine_count(positions2.clone(), 1);

            let diff = area2.difference(&area1);

            assert_eq!(diff, Area::new(diff_2_positions.clone(), 0..=0));
        }
    }

    #[test]
    fn area_difference_with_ranged_mine_count() {
        let positions1 = HashSet::from([
            Position::new(0, 1),
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
        ]);
        let positions2 = HashSet::from([
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(3, 0),
        ]);
        let diff_1_positions: HashSet<Position> =
            positions1.difference(&positions2).cloned().collect();
        let diff_2_positions: HashSet<Position> =
            positions2.difference(&positions1).cloned().collect();

        {
            let area1 = Area::new(positions1.clone(), 0..=2);
            let area2 = Area::new(positions2.clone(), 1..=2);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 0..=2));
        }
        {
            let area1 = Area::new(positions1.clone(), 0..=1);
            let area2 = Area::new(positions2.clone(), 0..=2);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::new(positions1.clone(), 1..=3);
            let area2 = Area::new(positions2.clone(), 0..=2);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 0..=2));
        }

        let positions3 = HashSet::from([
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(1, 0),
            Position::new(1, 2),
            Position::new(2, 0),
            Position::new(2, 2),
            Position::new(2, 1),
        ]);
        let positions4 = HashSet::from([
            Position::new(2, 0),
            Position::new(2, 2),
            Position::new(2, 1),
        ]);
        let diff_3_positions: HashSet<Position> =
            positions3.difference(&positions4).cloned().collect();
        let diff_4_positions: HashSet<Position> =
            positions4.difference(&positions3).cloned().collect();

        {
            let area1 = Area::new(positions3.clone(), 1..=1);
            let area2 = Area::new(positions4.clone(), 0..=1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_3_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::new(positions3.clone(), 1..=2);
            let area2 = Area::new(positions4.clone(), 0..=1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_3_positions.clone(), 0..=2));
        }
        {
            let area1 = Area::new(positions3.clone(), 2..=3);
            let area2 = Area::new(positions4.clone(), 0..=1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_3_positions.clone(), 1..=3));
        }
        {
            let area1 = Area::new(positions3.clone(), 2..=3);
            let area2 = Area::new(positions4.clone(), 1..=2);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_3_positions.clone(), 0..=2));
        }
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
            mine_count: match self.get_tile(pos) {
                Some(tile) => match tile.value() {
                    Value::Near(val) => {
                        MineCount::new(val as usize - flags_around, val as usize - flags_around)
                    }
                    Value::Mine => MineCount::new(0, 8),
                },
                None => MineCount::new(0, 8),
            },
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
