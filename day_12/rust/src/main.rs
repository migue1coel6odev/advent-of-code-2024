use std::fs;

use land::Land;

mod display_map;
mod land;
mod plot;

fn main() {
    let mut land = Land::new(read_file());
    land.map_plots();

    println!("Nr plots: {}", &land.calculate_fencing());
    println!("Nr plots: {}", &land.calculate_fencing_sides())
}

fn read_file() -> Vec<Vec<char>> {
    String::from_utf8(fs::read("test.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
