use std::fs;

use machine::Machine;
use regex::Regex;


mod machine;

fn main() {
    let vec_machines = read_file();

    let token_sum = vec_machines.iter().map(|f| f.calculate_best_token_usage()).sum::<usize>();

    println!("Tokens : {}", token_sum);
}

fn read_file() -> Vec<Machine> {
    let input =String::from_utf8(fs::read("test.txt").unwrap()).unwrap();

    let reg = Regex::new(r".+X.(?<a_x>\d+).+Y.(?<a_y>\d+)\n.+X.(?<b_x>\d+).+Y.(?<b_y>\d+)\n.+X.(?<prize_x>\d+).+Y.(?<prize_y>\d+)").unwrap();

    let captures = reg.captures_iter(&input);

    let mut result = Vec::new();

    for capture in captures {
        // let cap = capture.extract();
        let button_a = (capture["a_x"].parse::<usize>().unwrap(), capture["a_y"].parse::<usize>().unwrap());
        let button_b = (capture["b_x"].parse::<usize>().unwrap(), capture["b_y"].parse::<usize>().unwrap());
        let prize = (capture["prize_x"].parse::<usize>().unwrap(), capture["prize_y"].parse::<usize>().unwrap());
        result.push(Machine::new(button_a, button_b, prize));
    }

    result
}