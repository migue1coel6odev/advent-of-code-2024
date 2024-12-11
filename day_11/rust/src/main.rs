use std::{collections::HashMap, fs, ops::Add};

use magic_stones::MagicStones;
use magic_stones_v2::MagicStonesFast;
use magic_stones_v3::MagicStonesSuperFast;
use magic_stones_v4::MagicStonesSuperSuperFast;

mod magic_stones;
mod magic_stones_v2;
mod magic_stones_v3;
mod magic_stones_v4;

fn main() {
    // part_1();
    // part_2();
    // part_2_v2();
    part_2_v3();
}

fn part_2_v3() {
    let mut ms = MagicStonesSuperSuperFast::new(read_file_as_hashmap());
    println!("count = {}", ms.blink(75));
}

// fn part_2_v2() {
//     let mut mssf = MagicStonesSuperFast::new(read_file_as_string());
//     let count = mssf.blink(25);
//     println!("count: {}", count);
// }

// fn part_2() {
//     let input = read_file();
//     let mut ms = MagicStonesFast::new(input, 25);
//     let res = ms.blink(75);
//     println!("L: {}", res.len());
// }

fn part_1() {
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

    // println!("{}", stones.join(" "));
    println!("Length: {}", stones.len());
}

fn read_file() -> Vec<usize> {
    String::from_utf8(fs::read("input.txt").unwrap())
        .unwrap()
        .split(" ")
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn read_file_as_string() -> String {
    String::from_utf8(fs::read("test.txt").unwrap()).unwrap()
}

fn read_file_as_hashmap() -> HashMap<usize, usize> {
    let mut hashmap = HashMap::new();
    String::from_utf8(fs::read("input.txt").unwrap())
        .unwrap()
        .split(" ")
        .for_each(|f| {
            hashmap.insert(f.parse::<usize>().unwrap(), 1);
        });

    hashmap
}
