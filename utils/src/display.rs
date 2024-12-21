use std::fmt::Display;

use crate::{command::pause, coord::Coord};

pub fn display_char_matrix<A: std::fmt::Display>(matrix: Vec<Vec<A>>) {
    println!("");
    for line in matrix {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

pub fn display_matrix<A: std::fmt::Display>(matrix: &Vec<Vec<A>>) {
    println!("");
    for line in matrix {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

pub fn display_matrix_highlight<A: Display + PartialEq>(
    matrix: &Vec<Vec<A>>,
    highlight: Vec<(A, Vec<Coord>)>,
) {
    println!("");
    for (y, line) in matrix.iter().enumerate() {
        'x: for (x, c) in line.iter().enumerate() {
            for (character, vec) in &highlight {
                if vec.contains(&Coord { x, y }) {
                    print!("{}", character);
                    continue 'x;
                }
            }
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}
