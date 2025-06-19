use crate::grid::{Cell, Grid, HEIGHT, WIDTH};
use std::io::{self, Write};

pub fn display(grid: &Grid) {
    print!("\x1B[2J\x1B[1;1H"); // 清屏并移动光标到左上角

    // 打印顶部边框
    println!("+{}+", "-".repeat(WIDTH * 2));

    for row in 0..HEIGHT {
        print!("|");
        for col in 0..WIDTH {
            match grid.cells[row][col] {
                Cell::Empty => print!("  "),
                Cell::Obstacle => print!("\x1B[31m██\x1B[0m"),
                Cell::Player => print!("\x1B[32m⛄\x1B[0m"),
                Cell::Target => print!("\x1B[33m⭐\x1B[0m"),
                Cell::Path => print!("\x1B[34m··\x1B[0m"),
            }
        }
        println!("|");
    }

    // 打印底部边框
    println!("+{}+", "-".repeat(WIDTH * 2));
}

pub fn get_input() -> Option<(usize, usize)> {
    print!("请输入目标坐标 (例如: 5 10, 输入Q退出): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_uppercase();
    if input == "Q" {
        return None;
    }

    let coords: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if coords.len() != 2 {
        println!("请输入两个数字，用空格分隔");
        return None;
    }

    Some((coords[0], coords[1]))
}
