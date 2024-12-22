use std::collections::{HashMap, HashSet};

use aoc_utils::{
    command::pause,
    coord::{self, Coord},
    display::{self, display_matrix, display_matrix_from_vec_coord, display_matrix_highlight},
    find::find_coord_of_char_as_coord,
    matrix::matrix::CharMatrix,
    maze::{maze_v2::Mazev2, maze_v3::Mazev3},
};

use crate::maze::Maze;

pub struct Racev2 {
    race_map: Vec<Vec<char>>,
    max_cheat_depth: usize,
    save_at_least: usize,
    race_normal_path: Vec<Coord>,
}

impl Racev2 {
    pub fn new(race_map: Vec<Vec<char>>) -> Self {
        let start_pos = find_coord_of_char_as_coord(&race_map, &'S').unwrap();
        let end_pos = find_coord_of_char_as_coord(&race_map, &'E').unwrap();
        let maze = Mazev2::new(&race_map, &start_pos, &end_pos);
        Self {
            max_cheat_depth: 0,
            save_at_least: 0,
            race_normal_path: maze.find_fastest_path().unwrap(),
            race_map,
        }
    }

    pub fn check_paths_that_saves(&mut self) -> usize {
        let mut found_paths = 0;
        let mut current_map = self.race_map.clone();
        let next_coord_index = self.mark_positions_below_required_save_time(&mut current_map);
        let mut current_normal_path = self.race_normal_path.clone();
        let mut current_possible_with_required_save_time =
            self.get_next_slice_possible_coord(next_coord_index);

        while current_possible_with_required_save_time.len() > 0 {
            let coord = current_normal_path.remove(0);
            let (_, max_paths) = self.create_matrix_for_current_coord(&current_map, coord);

            found_paths += self.create_and_solve_maze_for_current_coord(
                &current_map,
                coord,
                &current_possible_with_required_save_time,
                max_paths,
            );
            let next_forbidden_coord = current_possible_with_required_save_time.remove(0);
            next_forbidden_coord.mark_coord_as_visited(&mut current_map);
        }

        found_paths
    }

    fn calculate_saved(&self, current_coord: &Coord, cheat_path: &Vec<Coord>) -> usize {
        let mut new_map = self.race_map.clone();

        let mut found = false;
        for coord in &self.race_normal_path {
            if found {
                coord.mark_coord_as_visited_with(&mut new_map, '.');
            } else {
                coord.mark_coord_as_visited(&mut new_map);
            }
            if coord == current_coord {
                found = true;
            }

            if coord == cheat_path.last().unwrap() {
                found = false;
            }
        }
        for coord in cheat_path {
            coord.mark_coord_as_visited_with(&mut new_map, '@');
        }
        self.race_normal_path
            .last()
            .unwrap()
            .mark_coord_as_visited_with(&mut new_map, 'E');
        self.race_normal_path
            .first()
            .unwrap()
            .mark_coord_as_visited_with(&mut new_map, 'S');

        let cheat_path_len: usize = new_map
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|c| *c == &'X' || *c == &'E' || *c == &'S' || *c == &'@')
                    .count()
            })
            .sum();

        if self.race_normal_path.len() - cheat_path_len == self.save_at_least {
            display_matrix(&new_map);
        }

        // let nr_left: usize = new_map
        //     .iter()
        //     .map(|line| line.iter().filter(|c| *c == &'.').count())
        //     .sum();

        self.race_normal_path.len() - cheat_path_len
    }

    fn display_vec(vec: &Vec<Coord>) {
        println!();
        for coord in vec {
            print!("{},{}", coord.x, coord.y);
            print!("|");
        }
    }

    fn create_matrix_for_current_coord(
        &self,
        map: &Vec<Vec<char>>,
        coord: Coord,
    ) -> (Vec<Coord>, usize) {
        CharMatrix::new(map.clone()).find_reachable_area(
            coord,
            self.max_cheat_depth,
            vec!['.', 'E', '#', 'X'],
            vec!['.', 'E'],
        )
    }

    fn create_and_solve_maze_for_current_coord(
        &self,
        map: &Vec<Vec<char>>,
        coord: Coord,
        current_possible_with_required_save_time: &Vec<Coord>,
        max_paths: usize,
    ) -> usize {
        Maze::new(&map, &coord, vec!['.', 'E'], vec!['#', 'X'], 'X').find_all_paths(
            max_paths,
            self.max_cheat_depth,
            current_possible_with_required_save_time.clone(),
        )
    }

    fn mark_positions_below_required_save_time(&self, mut map: &mut Vec<Vec<char>>) -> usize {
        let next_coord_index = self.save_at_least + 1;
        for coord in self.race_normal_path[1..next_coord_index].to_vec() {
            coord.mark_coord_as_visited(&mut map);
        }

        next_coord_index
    }

    fn get_next_slice_possible_coord(&self, next_coord_index: usize) -> Vec<Coord> {
        self.race_normal_path[next_coord_index..self.race_normal_path.len()].to_vec()
    }

    pub fn set_max_cheat_depth(&mut self, new_max_cheat_depth: usize) {
        self.max_cheat_depth = new_max_cheat_depth;
    }

    pub fn set_save_at_least(&mut self, new_save_at_least: usize) {
        self.save_at_least = new_save_at_least;
    }
}

#[cfg(test)]
mod test {
    use aoc_utils::{coord::Coord, display::display_matrix};

    use super::Racev2;

    // #[test]
    // fn test_race_v2() {
    //     let mut input: Vec<Vec<char>> = "
    //     ###############
    //     #...#...#.....#
    //     #.#.#.#.#.###.#
    //     #S#...#.#.#...#
    //     #######.#.#.###
    //     #######.#.#...#
    //     #######.#.###.#
    //     ###..E#...#...#
    //     ###.#######.###
    //     #...###...#...#
    //     #.#####.#.###.#
    //     #.#...#.#.#...#
    //     #.#.#.#.#.#.###
    //     #...#...#...###
    //     ###############"
    //         .to_string()
    //         .trim()
    //         .split("\n")
    //         .map(|line| line.trim().chars().collect::<Vec<char>>())
    //         .collect::<Vec<Vec<char>>>();

    //     let mut res = input.clone();
    //     res[2][1] = 'X';
    //     res[1][1] = 'X';

    //     let mut race = Racev2::new(input.clone());
    //     race.set_max_cheat_depth(6);
    //     race.set_save_at_least(2);
    //     let next_coord_index = race.mark_positions_below_required_save_time(&mut input);
    //     let current_possible_with_required_save_time =
    //         race.get_next_slice_possible_coord(next_coord_index);

    //     assert_eq!(input, res);
    //     assert_eq!(next_coord_index, 3);
    //     assert_eq!(
    //         current_possible_with_required_save_time[0],
    //         Coord { x: 2, y: 1 }
    //     );
    //     assert_eq!(race.race_normal_path[0], Coord { x: 1, y: 3 });
    //     assert_eq!(race.race_normal_path.last().unwrap(), &Coord { x: 5, y: 7 });
    // }
}
