#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    value: Value,
    state: State,
}

impl Tile {
    pub fn new(value: Value, state: State) -> Self {
        Self { value, state }
    }

    pub fn with_value(value: Value) -> Self {
        Self {
            value,
            state: Default::default(),
        }
    }

    pub fn increment_value(&mut self) {
        if let Value::Near(value) = &mut self.value {
            *value += 1;
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    // Tells the number of mines near the tile.
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
                Value::Mine => "#".to_string(),
            }
        )
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Near(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Closed,
    Open,
    Flag,
}

impl State {
    pub fn open(&self) -> Option<Self> {
        if *self == State::Closed {
            Some(State::Open)
        } else {
            None
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Closed
    }
}
