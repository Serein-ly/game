use crate::grid::{Cell, Grid, HEIGHT, WIDTH};
use crate::pathfinder::find_path;
use crate::ui::{display, get_input};
use rand::Rng;
use std::thread;
use std::time::Duration;

pub struct Game {
    pub grid: Grid,
    pub player_pos: (usize, usize),
    pub target_pos: Option<(usize, usize)>,
}

impl Game {
    pub fn new() -> Self {
        let mut grid = Grid::new();
        let mut rng = rand::rng();

        // 放置玩家
        let player_pos = loop {
            let pos = (rng.random_range(0..HEIGHT), rng.random_range(0..WIDTH));
            if grid.cells[pos.0][pos.1] == Cell::Empty {
                grid.cells[pos.0][pos.1] = Cell::Player;
                break pos;
            }
        };

        Game {
            grid,
            player_pos,
            target_pos: None,
        }
    }

    pub fn set_target(&mut self, x: usize, y: usize) -> bool {
        if x >= HEIGHT || y >= WIDTH {
            return false;
        }

        if self.grid.cells[x][y] == Cell::Obstacle {
            return false;
        }

        if let Some(old_target) = self.target_pos {
            self.grid.cells[old_target.0][old_target.1] = Cell::Empty;
        }

        self.grid.cells[x][y] = Cell::Target;
        self.target_pos = Some((x, y));

        true
    }

    pub fn animate_movement(&mut self, path: Vec<(usize, usize)>) {
        // 显示路径
        for &pos in &path {
            if self.grid.cells[pos.0][pos.1] == Cell::Empty {
                self.grid.cells[pos.0][pos.1] = Cell::Path;
            }
        }

        // 移动玩家
        let mut current_pos = self.player_pos;
        for &next_pos in &path {
            // 移动玩家
            self.grid.cells[current_pos.0][current_pos.1] = Cell::Empty;
            current_pos = next_pos;
            self.grid.cells[current_pos.0][current_pos.1] = Cell::Player;
            self.player_pos = current_pos;

            // 显示动画
            display(&self.grid);
            thread::sleep(Duration::from_millis(300));
        }

        // 清除路径显示
        self.grid.clear_paths();
    }

    pub fn run(&mut self) {
        loop {
            display(&self.grid);

            if let Some(target) = self.target_pos {
                if self.player_pos == target {
                    println!("到达目标位置！");
                    thread::sleep(Duration::from_secs(1));
                    self.target_pos = None;
                    continue;
                }
            }

            match get_input() {
                Some((x, y)) => {
                    if !self.set_target(x, y) {
                        println!("无效的目标位置");
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }

                    if let Some(target) = self.target_pos {
                        if let Some(path) = find_path(&self.grid.cells, self.player_pos, target) {
                            self.animate_movement(path);
                        } else {
                            println!("无法找到路径到目标位置");
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                }
                None => {
                    println!("游戏结束");
                    break;
                }
            }
        }
    }
}
