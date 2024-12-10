use std::fs;

use hiking::Hiking;

mod hiking;

fn main() {
    let input = read_file();
    let mut hiking_heads = Hiking::get_trailheads_as_hiking(input);
    let sum: usize = hiking_heads
        .iter_mut()
        .map(|hiking| hiking.discover())
        .sum::<usize>();
    println!(
        "Sum: {} with rating: {}",
        sum,
        hiking_heads
            .iter()
            .map(|hiking| hiking.rating)
            .sum::<usize>()
    );

    // let mut hikings = &mut Hiking::get_trailheads_as_hiking(input);
    // let s = hikings[0].discover();
    // println!("Sum: {:?} {}", hikings[0].head, s);
}

fn read_file() -> Vec<Vec<usize>> {
    String::from_utf8(fs::read("input.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|v| v.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
