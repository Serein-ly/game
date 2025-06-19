use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Obstacle,
    Player,
    Target,
    Path,
}

pub const WIDTH: usize = 20;
pub const HEIGHT: usize = 10;

pub struct Grid {
    pub cells: [[Cell; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        let mut cells = [[Cell::Empty; WIDTH]; HEIGHT];
        let mut rng = rand::rng();

        // 随机生成障碍物 (大约20%的格子)
        for row in cells.iter_mut() {
            for cell in row.iter_mut() {
                if rng.random_range(0..100) < 20 {
                    *cell = Cell::Obstacle;
                }
            }
        }

        Grid { cells }
    }

    pub fn clear_paths(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == Cell::Path {
                    *cell = Cell::Empty;
                }
            }
        }
    }
}
