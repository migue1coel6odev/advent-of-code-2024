use std::{collections::{HashMap, HashSet}, fs};

use antenna::Antenna;
use antenna_v2::Antennav2;

mod antenna;
mod antenna_v2;

fn main() {
    let input = read_file();
    // part_1(&input);
    part_2(&input);

}

fn part_2(input: &Vec<Vec<char>>){
    let mut antenna_manager: HashMap<char, Antennav2> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != '.' {
                antenna_manager
                    .entry(c)
                    .or_insert(Antennav2::new(c, line.len(), input.len()))
                    .add_antenna_pos((x, y));
            }
        }
    }

    let mut input_with_antinodes = input.clone();
    let mut unique_antinodes: HashSet<(usize, usize)> =  HashSet::new();
    

    for (_, mut antenna) in antenna_manager {
        // println!("{}", antenna);
        let antinodes = antenna.generate_antinodes();

        for antinode in antinodes {
            unique_antinodes.insert(*antinode);
        }
        // println!("{}", antenna);
    }

    for (x, y) in &unique_antinodes {
        if input_with_antinodes[*y][*x] == '.' {
            input_with_antinodes[*y][*x] = '#';
            continue;
        }
    }
    
    print_map(&input_with_antinodes);
    println!("| PART 2 | Unique antinodes = {}", unique_antinodes.len());
}

fn part_1(input: &Vec<Vec<char>>) {
    let mut antenna_manager: HashMap<char, Antenna> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != '.' {
                antenna_manager
                    .entry(c)
                    .or_insert(Antenna::new(c, line.len(), input.len()))
                    .add_antenna_pos((x, y));
            }
        }
    }

    let mut input_with_antinodes = input.clone();
    let mut unique_antinodes: HashSet<(usize, usize)> =  HashSet::new();
    

    for (_, mut antenna) in antenna_manager {
        // println!("{}", antenna);
        let antinodes = antenna.generate_antinodes();

        for antinode in antinodes {
            unique_antinodes.insert(*antinode);
        }
        // println!("{}", antenna);
    }

    for (x, y) in &unique_antinodes {
        if input_with_antinodes[*y][*x] == '.' {
            input_with_antinodes[*y][*x] = '#';
            continue;
        }
    }
    
    print_map(&input_with_antinodes);
    println!("| PART 1 | Unique antinodes = {}", unique_antinodes.len());
}

fn read_file() -> Vec<Vec<char>> {
    String::from_utf8(fs::read("input.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn print_map (map: &Vec<Vec<char>>){

    for line in map {
        for c in line {
            print!("{}", c);
        }
        print!("\n")
    }

}