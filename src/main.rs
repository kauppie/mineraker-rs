use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub enum Tile {
    // Tells the number of mines near the tile.
    Near(u8),
    Mine,
}

impl Tile {
    pub fn is_mine(&self) -> bool {
        match *self {
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
    pub fn new(width: usize, height: usize, mines: usize, _seed: usize) -> Self {
        // Assert mine count doesn't exceed the number of tiles.
        assert!(mines <= width * height);

        // Add all tiles where mines first reside in the front.
        let mut tiles = Vec::with_capacity(width * height);
        for _ in 0..mines {
            tiles.push(Tile::Mine);
        }
        for _ in 0..(width * height - mines) {
            tiles.push(Tile::Near(0));
        }
        // Shuffle mines around.
        let rng = &mut rand::thread_rng();
        tiles.shuffle(rng);

        let board = Self { tiles, width };
        let mut numbered = board.clone();

        // Number tiles which are close to mines.
        for (i, tile) in board.tiles.iter().enumerate() {
            if tile.is_mine() {
                let neighbors = board.tile_neighbors((i % board.width, i / board.width));
                neighbors.into_iter().for_each(|xy| {
                    // Unwrap as these coordinates are directly from enumeration.
                    if let Tile::Near(val) = numbered.get_tile_mut(xy).unwrap() {
                        *val += 1;
                    }
                });
            }
        }

        numbered
    }

    #[allow(dead_code)]
    fn pos_iter(size: usize, width: usize) -> impl Iterator<Item = (usize, usize)> + 'static {
        (0..size).map(move |i| (i % width, i / width))
    }

    pub fn tile_neighbors(&self, xy: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (x, y) = (xy.0, xy.1);

        // Use wrapping_sub to wrap around to usize::MAX on zero values which is always filtered out.
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

fn main() {
    let board = Board::new(30, 16, 170, 0);

    println!("{}", board);
}
