use std::fs;

use guard_path::GuardPath;

mod guard_path;

fn main() {
    let input = read_file();

    let guard_path = GuardPath::new(input);
    guard_path.calculate_guard_path();
    guard_path.calculate_possible_guard_loops(); // 1705
}

fn read_file() -> Vec<Vec<char>> {
    String::from_utf8(fs::read("input.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}
