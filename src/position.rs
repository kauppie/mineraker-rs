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

    /// Returns iterator over [`Position`]s neighbor [`Position`]s. Filters out positions which
    /// are outside given width and height bounds. Positions are returned in row-major order
    /// starting lower bound of coordinates.
    ///
    /// # Examples
    /// ```
    /// use mineraker::position::Position;
    ///
    /// // Position which is against the lower horizontal bound.
    /// let position = Position::new(1, 0);
    /// let neighbors: Vec<Position> = position.neighbors(8, 8).collect();
    ///
    /// assert_eq!(neighbors, [
    ///     Position::new(0, 0),
    ///     Position::new(2, 0),
    ///     Position::new(0, 1),
    ///     Position::new(1, 1),
    ///     Position::new(2, 1),
    /// ]);
    /// ```
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

impl From<(usize, usize)> for Position {
    /// Convert tuple with x and y coordinates into a position.
    ///
    /// # Examples
    /// ```
    /// use mineraker::position::Position;
    ///
    /// let pos1 = Position::new(1, 2);
    /// let pos2 = Position::from((1, 2));
    ///
    /// assert_eq!(pos1, pos2);
    /// ```
    fn from(xy: (usize, usize)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}
