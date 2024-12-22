use std::collections::{HashMap, HashSet};

use aoc_utils::{
    command::pause,
    coord::{self, Coord},
    display::{self, display_matrix, display_matrix_highlight},
    find::find_coord_of_char_as_coord,
    matrix::matrix::CharMatrix,
    maze::{maze_v2::Mazev2, maze_v3::Mazev3},
};

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
        let mut mark_next = self.save_at_least + self.max_cheat_depth - 1;
        for coord in self.race_normal_path[0..mark_next].to_vec() {
            coord.mark_coord_as_visited(&mut new_map);
        }

        let mut current_normal_path =
            self.race_normal_path.clone()[0..self.race_normal_path.len() - mark_next].to_vec();

        while current_normal_path.len() > 0 {
            self.race_normal_path[mark_next].mark_coord_as_visited(&mut new_map);
            mark_next += 1;
            let coord = current_normal_path.remove(0);
            let cm = CharMatrix::new(new_map.clone());

            let maze = Mazev3::new(
                &new_map,
                &coord,
                '.',
                vec!['#'],
                'X',
                cm.find_reachable_area(coord, 20),
            );
            let res = maze.find_all_paths().len();

            // self.check_next_pos(&new_map, &coord);
        }

        self.possible_cheats.len()
    }

    fn check_next_pos(&mut self, current_map: &Vec<Vec<char>>, current_coord: &Coord) {
        let surroundings = current_coord.check_surroundings(&current_map, vec!['#']);

        let initial_val = self.possible_cheats.len();
        for (_, coord) in surroundings {
            self.check_for_cheats(current_map, current_coord, coord, 1);
        }
        if initial_val != self.possible_cheats.len() {
            self.cache_dead_end.push(*current_coord);
            println!("current cache size: {}", self.cache_dead_end.len());
        }
    }

    fn check_for_cheats(
        &mut self,
        current_map: &Vec<Vec<char>>,
        original_coord: &Coord,
        current_coord: Coord,
        current_depth: usize,
    ) {
        if current_depth == self.max_cheat_depth || self.cache_dead_end.contains(&current_coord) {
            return ();
        }

        let surroundings = current_coord.check_surroundings(&current_map, vec!['#', '.', 'E']);

        for (_, coord) in surroundings {
            if coord.check_char_at(&current_map) != '#' {
                self.possible_cheats.insert((*original_coord, coord));
            } else {
                let mut new_map = current_map.clone();
                coord.mark_coord_as_visited(&mut new_map);
                self.check_for_cheats(&new_map, original_coord, coord, current_depth + 1);
            }
        }
    }

    pub fn display_possible_cheats(&self) {
        for (start_cheat, end_cheat) in &self.possible_cheats {
            let mut map = self.race_map.clone();
            start_cheat.mark_coord_as_visited(&mut map);
            end_cheat.mark_coord_as_visited(&mut map);
            display_matrix(&map);
            pause();
        }
    }
}
