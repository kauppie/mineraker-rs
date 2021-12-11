use std::{collections::HashSet, ops::RangeInclusive};

use crate::position::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MineCount(RangeInclusive<usize>);

impl MineCount {
    pub fn from_exact(exact: usize) -> Self {
        Self(exact..=exact)
    }

    pub fn from_range(min: usize, max: usize) -> Self {
        Self(min..=max)
    }

    #[inline]
    pub fn min(&self) -> usize {
        *self.0.start()
    }

    #[inline]
    pub fn max(&self) -> usize {
        *self.0.end()
    }

    /// Returns the exact number of mines if min and max are equal, [`None`] otherwise.
    ///
    /// # Examples
    /// ```
    /// use mineraker::area::MineCount;
    ///
    /// assert_eq!(MineCount::from_exact(3).exact_count(), Some(3));
    /// assert_eq!(MineCount::from_range(1, 1).exact_count(), Some(1));
    /// assert_eq!(MineCount::from_range(1, 2).exact_count(), None);
    /// ```
    #[inline]
    pub fn exact_count(&self) -> Option<usize> {
        let min = self.min();
        if min == self.max() {
            Some(min)
        } else {
            None
        }
    }
}

impl From<RangeInclusive<usize>> for MineCount {
    fn from(ri: RangeInclusive<usize>) -> Self {
        Self(ri)
    }
}

impl From<usize> for MineCount {
    fn from(exact: usize) -> Self {
        Self::from_exact(exact)
    }
}

/// Stores available action for [`Area`]. Some [`Area`]s do not
/// have available actions.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AreaAction {
    Open,
    Flag,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    positions: HashSet<Position>,
    // Stores the number of mines area contains.
    mine_count: MineCount,
}

impl Area {
    /// Creates a new [`Area`] with the given positions and mine count.
    ///
    /// # Examples
    /// ```
    /// use mineraker::area::{Area, MineCount};
    /// use mineraker::position::Position;
    ///
    /// let positions = [
    ///     Position::new(4, 4),
    ///     Position::new(5, 5),
    /// ];
    ///
    /// // Mine count as single integer.
    /// let area = Area::new(positions.into(), 1);
    ///
    /// // Mine count as range with integers.
    /// let area2 = Area::new(positions.into(), 1..=2);
    ///
    /// // Mine count via `MineCount` construct function.
    /// let area3 = Area::new(positions.into(), MineCount::from_range(0, 2));
    /// ```
    pub fn new(positions: HashSet<Position>, mine_count: impl Into<MineCount>) -> Self {
        Self {
            positions,
            mine_count: mine_count.into(),
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
    /// let area1 = Area::new(HashSet::from([Position::new(0, 0), Position::new(1, 0)]), 2);
    /// let area2 = Area::new(HashSet::from([Position::new(1, 0)]), 1);
    ///
    /// assert_eq!(area1.difference(&area2), Area::new(HashSet::from([Position::new(0, 0)]), 1));
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let diff: HashSet<_> = self
            .positions
            .difference(&other.positions)
            .cloned()
            .collect();

        let intersection_size = self.positions.intersection(&other.positions).count();

        let min = {
            let intersection_mines = intersection_size
                .min(self.mine_count.min())
                .min(other.mine_count.max());

            // This can't underflow as `intersection_mines` equal to or smaller than
            // `self.mine_count.min()` based on previous expression.
            self.mine_count.min() - intersection_mines
        };
        let max = {
            // Can't underflow as intersection is always equal to or smaller than
            // the area that forms it.
            let other_diff_size = other.positions.len() - intersection_size;
            // Use `saturating_sub` to emulate calcuting max between result and 0.
            let other_mines_overflow_to_intersection =
                other.mine_count.min().saturating_sub(other_diff_size);

            // Substraction can't underflow as `self.mine_count.max()` includes
            // mines that could possibly be in the intersection area and therefore
            // it is always greater or equal to mine count in the intersection.
            diff.len()
                .min(self.mine_count.max() - other_mines_overflow_to_intersection)
        };

        Self {
            positions: diff,
            mine_count: MineCount::from_range(min, max),
        }
    }

    /// Returns the next possible action for [`Area`] if one exists.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashSet;
    /// use mineraker::area::{Area, AreaAction};
    /// use mineraker::position::Position;
    ///
    /// let positions = HashSet::from([
    ///     Position::new(1, 0),
    ///     Position::new(2, 0),
    ///     Position::new(3, 0),
    /// ]);
    ///
    /// let area = Area::new(positions.clone(), 0);
    /// assert_eq!(area.next_action(), Some(AreaAction::Open));
    /// let area2 = Area::new(positions.clone(), 3);
    /// assert_eq!(area2.next_action(), Some(AreaAction::Flag));
    /// let area3 = Area::new(positions.clone(), 1);
    /// assert_eq!(area3.next_action(), None);
    /// let area4 = Area::new(positions.clone(), 1..=3);
    /// assert_eq!(area4.next_action(), None);
    /// ```
    pub fn next_action(&self) -> Option<AreaAction> {
        self.mine_count.exact_count().and_then(|count| match count {
            0 => Some(AreaAction::Open),
            _ if count == self.positions.len() => Some(AreaAction::Flag),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        area::{Area, MineCount},
        position::Position,
    };
    use std::collections::HashSet;

    #[test]
    fn area_creation_equivalence() {
        let area1 = Area::new(Default::default(), MineCount::from_exact(1));
        let area2 = Area::new(Default::default(), 1);

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
            let area1 = Area::new(positions1.clone(), 1);
            let area2 = Area::new(positions2.clone(), 1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::new(positions1.clone(), 2);
            let area2 = Area::new(positions2.clone(), 1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 1..=2));
        }
        {
            let area1 = Area::new(positions1.clone(), 3);
            let area2 = Area::new(positions2.clone(), 1);

            let diff = area1.difference(&area2);

            assert_eq!(diff, Area::new(diff_1_positions.clone(), 2..=2));
        }
        {
            let area1 = Area::new(positions1.clone(), 1);
            let area2 = Area::new(positions2.clone(), 1);

            let diff = area2.difference(&area1);

            assert_eq!(diff, Area::new(diff_2_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::new(positions1.clone(), 2);
            let area2 = Area::new(positions2.clone(), 1);

            let diff = area2.difference(&area1);

            assert_eq!(diff, Area::new(diff_2_positions.clone(), 0..=1));
        }
        {
            let area1 = Area::new(positions1.clone(), 3);
            let area2 = Area::new(positions2.clone(), 1);

            let diff = area2.difference(&area1);

            assert_eq!(
                diff,
                Area::new(diff_2_positions.clone(), MineCount::from_exact(0))
            );
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
