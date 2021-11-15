use std::{collections::HashSet, ops::RangeInclusive};

use crate::position::Position;

pub type MineCount = RangeInclusive<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    positions: HashSet<Position>,
    // Stores the number of mines area contains.
    mine_count: MineCount,
}

impl Area {
    /// Creates a new [`Area`] with the given positions and a mine count range.
    pub fn new(positions: HashSet<Position>, mine_count: MineCount) -> Self {
        Self {
            positions,
            mine_count,
        }
    }

    /// Creates a new [`Area`] with the given positions and a specific number of mines.
    ///
    /// # Examples
    /// ```
    /// use mineraker::area::Area;
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

    /// Calculates set difference between two [`Area`]s and returns area from `self` which is not
    /// in `other`.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashSet;
    /// use mineraker::area::{Area, MineCount};
    /// use mineraker::position::Position;
    ///
    /// let area1 = Area::with_definite_mine_count(HashSet::from([Position::new(0, 0), Position::new(1, 0)]), 2);
    /// let area2 = Area::with_definite_mine_count(HashSet::from([Position::new(1, 0)]), 1);
    ///
    /// assert_eq!(area1.difference(&area2), Area::with_definite_mine_count(HashSet::from([Position::new(0, 0)]), 1));
    /// ```
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
            // Use `saturating_sub` to limit value to zero with unsigned integers.
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

    use crate::{area::Area, position::Position};
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
