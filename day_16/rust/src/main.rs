use std::collections::HashMap;

use aoc_utils::{display::display_matrix, file_read::read_file_as_char_matrix};
use maze::Maze;
use maze_v2::Mazev2;
use maze_v3::Mazev3;
use maze_v4::Mazev4;

mod constants;
mod maze;
mod maze_2;
mod maze_v2;
mod maze_v3;
mod maze_v4;

fn main() {
    // part_1("input.txt");
    part_1_1("input.txt");
}

#[allow(dead_code)]
fn part_1(path: &str) -> usize {
    let map = read_file_as_char_matrix(path);

    let mut hashmap = HashMap::new();
    let mut count: Option<usize> = None;

    let mut maze = Maze::new(&map, constants::Facing::EAST, &mut hashmap, &mut count);

    if let Some(val) = maze.find_best_path() {
        println!("val {}", val);
        // println!("count: {}", &count);
        // println!("hashmap: {:#?}", &hashmap);
        return val;
    }
    0
}

#[allow(dead_code)]
fn part_1_1(path: &str) -> usize {
    let map = read_file_as_char_matrix(path);

    let mut maze = Mazev4::new(&map, constants::Facing::EAST);
    let vec = maze.find_best_path();

    if !vec.is_empty() {
        let min = vec.iter().min().unwrap();
        println!("vec = {:?}", vec);
        println!("Min = {}", min);
        return *min;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_1() {
    //     let result = part_1("test.txt");
    //     assert_eq!(result, 7036);
    // }

    #[test]
    fn test_part_1_test() {
        let result = part_1_1("test.txt");
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part_1_test_1() {
        let result = part_1_1("test1.txt");
        assert_eq!(result, 11048);
    }

    // #[test]
    // fn test_part_2() {
    //     let result = part_2("test.txt");
    //     assert_eq!(result, 9021);
    // }
}
