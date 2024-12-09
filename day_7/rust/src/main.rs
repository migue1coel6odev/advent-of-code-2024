use std::{collections::VecDeque, fs};

use equation::Equation;
use equation_v2::Equationv2;
use regex::Regex;

mod equation;
mod equation_v2;

fn main() {
    let (equations_v1, equations_v2) = read_file();

    let sum = equations_v1.iter().map(|equation| equation.resolve()).sum::<usize>();
    println!("| PART 1 | Sum: {}", sum);
    
    let sum_v2 = equations_v2.iter().map(|equation| equation.resolve()).sum::<usize>();
    println!("| PART 2 | Sum: {}", sum_v2);
}

fn read_file() -> (Vec<Equation>, Vec<Equationv2>) {
    let reg = Regex::new(r"(\d+)").unwrap();
    String::from_utf8(fs::read("input.txt").unwrap()).unwrap().split("\n").into_iter().map(|line| {
        let mut vec: VecDeque<usize> = VecDeque::new();
        for (_, [id]) in reg.captures_iter(&line).map(|nr| nr.extract()) {
            vec.push_back(id.parse::<usize>().unwrap());
        }

        let result: usize = VecDeque::pop_front(&mut vec).unwrap();

        (Equation::new(result, vec.clone().try_into().unwrap()),
        Equationv2::new(result, vec.clone().try_into().unwrap()))

    }).collect()

}