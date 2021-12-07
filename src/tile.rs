#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    value: Value,
    state: State,
}

impl Tile {
    /// Constant [`Tile`] which empty and closed.
    pub const EMPTY_CLOSED: Self = Self {
        value: Value::Near(0),
        state: State::Closed,
    };

    /// Constructs a new [`Tile`] with the given value and state.
    #[allow(dead_code)]
    pub fn new(value: Value, state: State) -> Self {
        Self { value, state }
    }

    /// Constructs a new [`Tile`] with given value and [`Default`] state.
    ///
    /// # Examples
    /// ```
    /// use mineraker::tile::{Tile, Value};
    ///
    /// let tile = Tile::with_value(Value::Near(3));
    /// assert_eq!(tile, Tile::new(Value::Near(3), Default::default()));
    /// ```
    #[inline]
    pub fn with_value(value: Value) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    /// Increments [`Tile`]s value if it is not a mine. Useful in board generation.
    ///
    /// # Examples
    /// ```
    /// use mineraker::tile::{Tile, Value};
    ///
    /// let mut tile = Tile::default();
    /// tile.increment_value();
    /// assert_eq!(tile, Tile::with_value(Value::Near(1)));
    ///
    /// // Tiles with Value::Mine value won't be affected by calling this function.
    /// let mut mine = Tile::with_value(Value::Mine);
    /// mine.increment_value();
    /// assert_eq!(mine, Tile::with_value(Value::Mine));
    /// ```
    #[inline]
    pub fn increment_value(&mut self) {
        if let Value::Near(value) = &mut self.value {
            *value += 1;
        }
    }

    /// Opens this tile, but only if it is currently closed.
    #[inline]
    pub fn open(&mut self) {
        if self.state == State::Closed {
            self.state = State::Open;
        }
    }

    /// Opens this tile, but only if it is currently closed.
    #[allow(dead_code)]
    #[inline]
    pub fn flag(&mut self) {
        if self.state == State::Closed {
            self.state = State::Flag;
        }
    }

    /// Toggles flag state of this tile. Closed tiles will be set to flag, flag tiles will be set closed.
    /// Open tiles won't change state.
    ///
    /// # Examples
    /// ```
    /// use mineraker::tile::{State, Tile, Value};
    ///
    /// let mut closed = Tile::new(Default::default(), State::Closed);
    /// closed.toggle_flag();
    /// assert_eq!(closed, Tile::new(Default::default(), State::Flag));
    ///
    /// let mut flag = Tile::new(Default::default(), State::Flag);
    /// flag.toggle_flag();
    /// assert_eq!(flag, Tile::new(Default::default(), State::Closed));
    ///
    /// let mut open = Tile::new(Default::default(), State::Open);
    /// open.toggle_flag();
    /// assert_eq!(open, Tile::new(Default::default(), State::Open));
    /// ```
    #[allow(dead_code)]
    #[inline]
    pub fn toggle_flag(&mut self) {
        self.state = match self.state {
            State::Closed => State::Flag,
            State::Open => State::Open,
            State::Flag => State::Closed,
        };
    }

    /// Returns the value of tile.
    #[inline]
    pub fn value(&self) -> Value {
        self.value
    }

    /// Returns the state of tile.
    #[allow(dead_code)]
    #[inline]
    pub fn state(&self) -> State {
        self.state
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.state {
                State::Closed => "#".to_string(),
                State::Open => self.value.to_string(),
                State::Flag => "?".to_string(),
            }
        )
    }
}

/// Value of a [`Tile`]. Value is either mine or number from 0 to 8, which represents the
/// number of mines around the tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Near(u8),
    Mine,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Near(0) => "_".to_string(),
                Value::Near(val) => val.to_string(),
                Value::Mine => "*".to_string(),
            }
        )
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Near(0)
    }
}

/// State of [`Tile`] which is one of the following states: closed, open or flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Closed,
    Open,
    Flag,
}

impl Default for State {
    fn default() -> Self {
        State::Closed
    }
}
