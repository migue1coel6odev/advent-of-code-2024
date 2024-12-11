use std::fs;

use magic_stones::MagicStones;

mod magic_stones;

fn main() {
    let input = read_file();

    let mut stones = vec![];

    for stone in input {
        let mut magic_stone = MagicStones::new(stone);
        let res = magic_stone.blink(25);
        stones.push(res);
    }

    let stones = stones
        .iter()
        .flat_map(|f| f)
        .map(|f| f.to_string())
        .collect::<Vec<_>>();

    println!("{}", stones.join(" "));
    println!("Length: {}", stones.len());
}

fn read_file() -> Vec<usize> {
    String::from_utf8(fs::read("test.txt").unwrap())
        .unwrap()
        .split(" ")
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
