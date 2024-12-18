use aoc_utils::file_read::read_file_as_char_matrix;
use maze::Maze;
use maze_2::Maze2;

mod constants;
mod maze;
mod maze_2;

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}

#[allow(dead_code)]
fn part_1(path: &str) -> usize {
    let map = read_file_as_char_matrix(path);

    let mut maze = Maze::new(&map, constants::Facing::EAST);
    let vec = maze.find_best_path();

    if !vec.is_empty() {
        let min = vec.iter().min().unwrap();
        println!("vec = {:?}", vec);
        println!("Min = {}", min);
        return *min;
    }

    0
}

#[allow(dead_code)]
fn part_2(path: &str) -> usize {
    let map = read_file_as_char_matrix(path);

    let mut maze = Maze2::new(&map, constants::Facing::EAST);
    let res = maze.find_best_path();

    println!("Result = {}", res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("test.txt");
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part_1_1() {
        let result = part_1("test1.txt");
        assert_eq!(result, 11048);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("test.txt");
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part_2_1() {
        let result = part_2("test1.txt");
        assert_eq!(result, 64);
    }
}
