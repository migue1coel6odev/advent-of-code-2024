use std::collections::{HashMap, HashSet};

use aoc_utils::{
    command::pause,
    coord::{self, Coord},
    display::{self, display_matrix, display_matrix_from_vec_coord, display_matrix_highlight},
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
        let mut found_paths = 0;

        let mut current_normal_path =
            self.race_normal_path.clone()[0..self.race_normal_path.len() - mark_next].to_vec();

        /**
         *
         * TODO:
         *
         * ter em conta que o maze nao esta a verificar se o resultado final so tem 20 steps
         * o maze esta a colocar em queue coordenadas duplicadas
         */
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
            let maze = Mazev3::new(
                &new_map,
                &coord,
                vec!['.', 'E'],
                vec!['#'],
                'X',
                restricted_coord.clone(),
            );
            if coord.x == 54 && coord.y == 49 {
                println!("here");
                display_matrix_highlight(&new_map, vec![('O', vec![coord])]);
                display_matrix_highlight(
                    &new_map,
                    vec![('O', vec![coord]), ('@', restricted_coord.clone())],
                );
                display_matrix_from_vec_coord(&new_map, restricted_coord);
                pause();
            }
            found_paths += maze.find_all_paths(max_paths).len();
        }

        found_paths
    }
}
