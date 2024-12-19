use aoc_utils::file_read::read_file_as_string;
use computer::Computer;
use regex::Regex;

mod computer;

fn main() {
    println!("| PART 1 | {}", part_1("input.txt"));
}

fn parse_input(input: &String) -> (usize, usize, usize, Vec<usize>) {
    let reg = Regex::new(
        r"Register A: (?<a>\d+)\nRegister B: (?<b>\d+)\nRegister C: (?<c>\d+)\n\nProgram: (?<p>[\d,]+)",
    ).unwrap();

    let caps = reg.captures(input).unwrap();

    (
        caps["a"].parse::<usize>().unwrap(),
        caps["b"].parse::<usize>().unwrap(),
        caps["c"].parse::<usize>().unwrap(),
        caps["p"]
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

#[allow(dead_code)]
fn part_1(path: &str) -> String {
    let input = read_file_as_string(path);
    let (register_a, register_b, register_c, program) = parse_input(&input);

    let mut computer = Computer::new(register_a, register_b, register_c, program);
    computer.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_input() {
    //     let input = read_file_as_string("test.txt");
    //     let parsed_input = parse_input(&input);
    //     assert_eq!(
    //         parsed_input,
    //         (729_usize, 0_usize, 0_usize, vec![0, 1, 5, 4, 3, 0])
    //     );
    // }

    #[test]
    fn test_part_1() {
        let result = part_1("test.txt");
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    // #[test]
    // fn test_part_2() {
    //     let result = part_2("test.txt");
    //     assert_eq!(result, 9021);
    // }
}
