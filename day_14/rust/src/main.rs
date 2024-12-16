use std::{collections::HashMap, thread, time::Duration};

use aoc_utils::{
    command::pause,
    display::{display_char_matrix, display_matrix},
    file_read::{read_file_as_string, read_file_by_line},
};
use regex::Regex;
use robot::Robot;

mod robot;

fn main() {
    // let result = part_1("input.txt", 101, 103);
    // println!("| PART 1 | = {} ", result);

    part_2("input.txt", 101, 103);
}

#[allow(dead_code)]
fn part_2(path: &str, tiles_wide: isize, tiles_tall: isize) {
    let input = read_file_as_string(path);

    let reg = Regex::new(r"p=(?<p_x>\d+),(?<p_y>\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)").unwrap();

    let captures = reg.captures_iter(&input);

    let mut robots = Vec::new();

    for capture in captures {
        robots.push(Robot::new(
            (
                capture["p_x"].parse::<isize>().unwrap(),
                capture["p_y"].parse::<isize>().unwrap(),
            ),
            capture["v_x"].parse::<isize>().unwrap(),
            capture["v_y"].parse::<isize>().unwrap(),
            tiles_wide,
            tiles_tall,
        ));
    }

    let mut nr_iterations = 100;

    loop {
        let mut robot_pos = vec![vec!['.'; tiles_wide as usize]; tiles_tall as usize];
        let mut top_left_quad = 0;
        let mut top_right_quad = 0;
        let mut bottom_left_quad = 0;
        let mut bottom_right_quad = 0;
        for robot in &robots {
            let (x, y) = &robot.pos_after(nr_iterations);
            robot_pos[*y as usize][*x as usize] = '#';

            if *x < tiles_wide / 2 {
                if *y < tiles_tall / 2 {
                    top_left_quad += 1;
                } else if *y > tiles_tall / 2 {
                    bottom_left_quad += 1;
                }
            } else if *x > tiles_wide / 2 {
                if *y < tiles_tall / 2 {
                    top_right_quad += 1;
                } else if *y > tiles_tall / 2 {
                    bottom_right_quad += 1;
                }
            }
        }

        for line in &robot_pos {
            if line
                .iter()
                .map(|f| if f == &'#' { 1 } else { 0 })
                .sum::<usize>()
                > 25
            {
                display_matrix(&robot_pos);
                // println!("nr iterations : {}", nr_iterations);
                // pause();
                thread::sleep(Duration::from_millis(100));
            }
        }

        nr_iterations += 1;
    }
}

#[allow(dead_code)]
fn part_1(path: &str, tiles_wide: isize, tiles_tall: isize) -> isize {
    let input = read_file_as_string(path);

    let reg = Regex::new(r"p=(?<p_x>\d+),(?<p_y>\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)").unwrap();

    let captures = reg.captures_iter(&input);

    let mut robots = Vec::new();

    for capture in captures {
        robots.push(Robot::new(
            (
                capture["p_x"].parse::<isize>().unwrap(),
                capture["p_y"].parse::<isize>().unwrap(),
            ),
            capture["v_x"].parse::<isize>().unwrap(),
            capture["v_y"].parse::<isize>().unwrap(),
            tiles_wide,
            tiles_tall,
        ));
    }

    let mut robot_pos = vec![vec![0; tiles_wide as usize]; tiles_tall as usize];
    let mut top_left_quad = 0;
    let mut top_right_quad = 0;
    let mut bottom_left_quad = 0;
    let mut bottom_right_quad = 0;
    for robot in robots {
        let (x, y) = robot.pos_after(100);
        robot_pos[y as usize][x as usize] += 1;

        if x < tiles_wide / 2 {
            if y < tiles_tall / 2 {
                top_left_quad += 1;
            } else if y > tiles_tall / 2 {
                bottom_left_quad += 1;
            }
        } else if x > tiles_wide / 2 {
            if y < tiles_tall / 2 {
                top_right_quad += 1;
            } else if y > tiles_tall / 2 {
                bottom_right_quad += 1;
            }
        }
    }

    display_char_matrix(robot_pos);

    println!(
        "{} | {} {} {} {}",
        tiles_tall / 2,
        top_left_quad,
        top_right_quad,
        bottom_left_quad,
        bottom_right_quad
    );

    top_left_quad * top_right_quad * bottom_left_quad * bottom_right_quad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part_1("test.txt", 11, 7);
        part_2("test.txt", 11, 7);
        assert_eq!(result, 12);
    }
}
