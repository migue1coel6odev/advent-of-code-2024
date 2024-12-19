use std::cmp::Ordering;

use aoc_utils::file_read::read_file_by_line;
use memory::Memory;

mod memory;

fn main() {
    println!("| PART 1 | Result = {}", part_1("input.txt", 1024));
    println!("| PART 2 | Result = {:?}", part_2("input.txt", 1024));
}

fn parse_input(lines: Vec<String>) -> (Vec<(usize, usize)>, usize) {
    let mut max = 0;
    (
        lines
            .iter()
            .map(|f| {
                let values = f
                    .split(",")
                    .map(|f| f.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                if values[0].cmp(&max) == Ordering::Greater {
                    max = values[0]
                }
                if values[1].cmp(&max) == Ordering::Greater {
                    max = values[1]
                }
                (values[0], values[1])
            })
            .collect::<Vec<(usize, usize)>>(),
        max,
    )
}

#[allow(dead_code)]
fn part_1(path: &str, nr_bytes: usize) -> usize {
    let (corrupted_pos, max_distance) = parse_input(read_file_by_line(path));

    let mut memory = Memory::new(&corrupted_pos, max_distance);
    memory.mark_corrupted_bytes(nr_bytes);
    memory.find_fastest_path().unwrap()
}

#[allow(dead_code)]
fn part_2(path: &str, nr_bytes: usize) -> (usize, usize) {
    let (corrupted_pos, max_distance) = parse_input(read_file_by_line(path));

    let mut memory = Memory::new(&corrupted_pos, max_distance);
    let mut nr_bytes_added = 0;
    memory.mark_corrupted_bytes(nr_bytes);
    loop {
        memory.mark_corrupted_bytes(1);
        nr_bytes_added += 1;
        if memory.find_fastest_path().is_none() {
            break;
        }
    }
    corrupted_pos[nr_bytes + nr_bytes_added - 1]
}

#[cfg(test)]
mod test {

    use super::*;

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(part_1("test.txt", 12), 22);
    // }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("test.txt", 12), (6, 1));
    }

    #[test]
    fn test_parse_input() {
        let (parsed_input, max) = parse_input(read_file_by_line("test.txt"));
        assert_eq!(parsed_input.len(), 25);
        assert_eq!(parsed_input[0], (5, 4));
        assert_eq!(max, 6);
    }
}
