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
            Tile::Near(_) => false,
            Tile::Mine => true,
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

#[derive(Debug, Default, Clone)]
pub struct Board {
    tiles: Vec<Tile>,
    width: usize,
}

impl Board {
    /// Generates a new board with the given width, height, mine count and seed.
    ///
    /// # Panics
    /// If `mines` > `width` * `height`.
    pub fn new(width: usize, height: usize, mines: usize, _seed: u64) -> Self {
        let size = width * height;
        // Assert mine count doesn't exceed the number of tiles.
        assert!(mines <= size);

        // Generate mine indexes.
        let rng = &mut rand::thread_rng();
        let indexes = rand::seq::index::sample(rng, size, mines);

        // Setup empty sized board.
        let empty_board = Self {
            tiles: {
                let mut vec = Vec::new();
                vec.resize(size, Tile::Near(0));
                vec
            },
            width,
        };
        let mut numbered = empty_board.clone();
        // Add mines and number tiles based on mine positions.
        indexes.iter().for_each(|idx| {
            numbered.tiles[idx] = Tile::Mine;
            // Increment number of all non-mine neighbors.
            empty_board
                .tile_neighbors((idx % width, idx / width))
                .for_each(|xy| {
                    // Unwrap as these coordinates are directly from enumeration and
                    // board and numbered are the same size.
                    if let Tile::Near(val) = numbered.get_tile_mut(xy).unwrap() {
                        *val += 1;
                    }
                });
        });

        numbered
    }

    #[allow(dead_code)]
    fn pos_iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.tiles.len()).map(move |i| (i % self.width, i / self.width))
    }

    /// Returns iterator over tile neighbor coordinates at the given coordinates.
    /// Excludes coordinates outside the board boundaries.
    pub fn tile_neighbors(&self, xy: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (x, y) = (xy.0, xy.1);

        // Use wrapping_sub to wrap around to usize::MAX on zero values which are always filtered out.
        [
            (x.wrapping_sub(1), y.wrapping_sub(1)),
            (x, y.wrapping_sub(1)),
            (x + 1, y.wrapping_sub(1)),
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x.wrapping_sub(1), y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter(|(x, y)| *x < self.width && *y < self.height())
    }

    pub fn get_tile_mut(&mut self, xy: (usize, usize)) -> Option<&mut Tile> {
        self.tiles.get_mut(xy.1 * self.width + xy.0)
    }

    pub fn get_tile(&self, xy: (usize, usize)) -> Option<&Tile> {
        self.tiles.get(xy.1 * self.width + xy.0)
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        if self.width != 0 {
            self.tiles.len() / self.width
        } else {
            0
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width {
                write!(f, "{}", self.get_tile((x, y)).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
