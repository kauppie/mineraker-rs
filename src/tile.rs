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
