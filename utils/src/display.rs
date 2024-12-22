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

pub fn display_matrix_from_vec_coord(original_matrix: &Vec<Vec<char>>, coords: Vec<Coord>) {
    let vec_x = coords.iter().map(|coord| coord.x).collect::<Vec<usize>>();
    let max_x = *vec_x.iter().max().unwrap();
    let min_x = *vec_x.iter().min().unwrap();

    let vec_y = coords.iter().map(|coord| coord.y).collect::<Vec<usize>>();
    let max_y = *vec_y.iter().max().unwrap();
    let min_y = *vec_y.iter().min().unwrap();

    let mut matrix = vec![vec![' '; (max_x - min_x) + 1]; (max_y - min_y) + 1];

    for coord in coords {
        let new_coord = Coord {
            x: coord.x - min_x,
            y: coord.y - min_y,
        };
        new_coord.mark_coord_as_visited_with(&mut matrix, coord.check_char_at(&original_matrix));
    }
    display_matrix(&matrix);
}
