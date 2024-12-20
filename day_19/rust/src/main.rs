use aoc_utils::file_read::read_file_as_splitted_by_two_new_lines;
use designs::Design;

mod designs;

fn main() {
    println!("| PART 1 | Result = {}", part_1("input.txt"));
    println!("| PART 2 | Result = {}", part_2("input.txt"));
}

#[allow(dead_code)]
fn part_1(path: &str) -> usize {
    let (available_patterns, desired_designs) = read_input(path);
    let design = Design::new(available_patterns);

    let result = desired_designs
        .iter()
        .filter(|desired_design| design.is_design_possible(desired_design))
        .collect::<Vec<&String>>();

    result.len()
}

#[allow(dead_code)]
fn part_2(path: &str) -> usize {
    let (available_patterns, desired_designs) = read_input(path);
    let design = Design::new(available_patterns);

    let result = desired_designs
        .iter()
        .filter(|desired_design| design.is_design_possible(desired_design))
        .collect::<Vec<&String>>();

    let result: usize = result
        .iter()
        .map(|desired_design| {
            let res = design.find_all_ways_possible(desired_design);
            println!("found {} ways for {}", res, desired_design);
            return res;
        })
        .sum();

    result
}

fn read_input(path: &str) -> (Vec<String>, Vec<String>) {
    let input = read_file_as_splitted_by_two_new_lines(path);
    (
        input[0]
            .split(",")
            .map(|f| f.trim().to_string())
            .collect::<Vec<String>>(),
        input[1]
            .split("\n")
            .map(|f| f.trim().to_string())
            .collect::<Vec<String>>(),
    )
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2, read_input};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("test.txt"), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("test.txt"), 16);
    }

    #[test]
    fn test_read_input() {
        let (available_patterns, desired_designs) = read_input("test.txt");
        assert_eq!(
            available_patterns,
            vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        );
        assert_eq!(
            desired_designs,
            vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",]
        )
    }
}
