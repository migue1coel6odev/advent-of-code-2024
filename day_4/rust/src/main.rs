use std::fs;

use find_x_mas::FindX_MAS;
use find_xmas::FindXMAS;

mod find_xmas;
mod find_x_mas;

fn main() {
    let text = read_file();

    let mut find_xmas: FindXMAS = FindXMAS::new(&text);
    let mut find_x_mas = FindX_MAS::new(&text);

    println!("count XMAS = {}", find_xmas.find_xmas());
    println!("count X-MAS = {}", find_x_mas.find_xmas());

}

fn read_file() -> Vec<String> {
    String::from_utf8(fs::read("input.txt").unwrap()).unwrap().split("\n").map(|line| line.to_string()).collect::<Vec<_>>()
}
