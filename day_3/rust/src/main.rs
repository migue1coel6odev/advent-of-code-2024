use std::fs;

use regex::Regex;

fn main(){
    part_one();
    part_two();
}

fn part_two() {
    let instructions = read_file();

    let sum = &instructions.split("do()").collect::<Vec<_>>().iter().map(|&c| {
        multiply_mul(c.split("don't()").collect::<Vec<_>>()[0])
    }).sum::<usize>();

    println!("| PART 2 | Sum: {}", sum);
}

fn part_one() {
    let instructions = read_file();

    let sum = multiply_mul(&instructions);

    println!("| PART 1 | Sum: {}", sum);
}

fn multiply_mul(instruction: &str) -> usize {
    let reg = Regex::new(r"(?<mul>mul\((?<leftOperand>[0-9]+),(?<rightOperand>[0-9]+)\))").unwrap();

    let sum = reg
        .captures_iter(instruction)
        .map(|c| {
            &c["leftOperand"].parse::<usize>().unwrap()
                * &c["rightOperand"].parse::<usize>().unwrap()
        })
        .sum::<usize>(); 


    sum
}

fn read_file() -> String {
    String::from_utf8(fs::read("input.txt").unwrap()).unwrap()
}
