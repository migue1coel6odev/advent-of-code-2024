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
    possible_cheats: HashSet<(Coord, Coord)>,
    cache_dead_end: Vec<Coord>,
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
            possible_cheats: HashSet::new(),
            max_cheat_depth: 0,
            save_at_least: 0,
            race_normal_path: maze.find_fastest_path().unwrap(),
            race_map,
            cache_dead_end: vec![],
        }
    }

    pub fn set_max_cheat_depth(&mut self, new_max_cheat_depth: usize) {
        self.max_cheat_depth = new_max_cheat_depth;
    }

    pub fn set_save_at_least(&mut self, new_save_at_least: usize) {
        self.save_at_least = new_save_at_least;
    }

    pub fn check_paths_that_saves(&mut self) -> usize {
        let mut new_map = self.race_map.clone();

        let mut mark_next = self.save_at_least;

        for coord in self.race_normal_path[0..mark_next].to_vec() {
            coord.mark_coord_as_visited(&mut new_map);
        }
        let mut found_paths = 0;

        let mut current_normal_path =
            self.race_normal_path.clone()[0..self.race_normal_path.len() - mark_next].to_vec();

        while current_normal_path.len() > 0 {
            self.race_normal_path[mark_next].mark_coord_as_visited(&mut new_map);
            mark_next += 1;

            let coord = current_normal_path.remove(0);
            let cm = CharMatrix::new(new_map.clone());
            let (restricted_coord, max_paths) = cm.find_reachable_area(
                coord,
                self.max_cheat_depth,
                vec!['.', 'E', '#', 'X'],
                vec!['.', 'E'],
            );
            let maze = Maze::new(
                &new_map,
                &coord,
                vec!['.', 'E'],
                vec!['#'],
                'X',
                restricted_coord.clone(),
            );

            let mut top = mark_next + self.max_cheat_depth;
            if top > self.race_normal_path.len() {
                top = self.race_normal_path.len();
            }

            let paths_found = maze.find_all_paths(
                max_paths,
                self.max_cheat_depth,
                self.race_normal_path[mark_next..top].to_vec(),
            );

            for vec in &paths_found {
                println!("saved: {}", self.calculate_saved(&coord, vec));
            }

            // display_matrix_highlight(&new_map, vec![('O', vec![coord])]);
            // display_matrix_highlight(
            //     &new_map,
            //     vec![('O', vec![coord]), ('@', restricted_coord.clone())],
            // );
            // display_matrix_from_vec_coord(&new_map, restricted_coord);
            // println!("paths found: {} coord: {:?}", paths_found, coord);
            // pause();
            found_paths += paths_found.len();
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
}
