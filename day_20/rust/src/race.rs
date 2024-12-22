use std::collections::HashMap;

use aoc_utils::{coord::Coord, find::find_coord_of_char_as_coord};

pub struct Race {
    race_map: Vec<Vec<char>>,
    possible_cheats: Vec<(Coord, Coord)>,
    time_util_coord: HashMap<Coord, usize>,
    start_pos: Coord,
}

impl Race {
    pub fn new(race_map: Vec<Vec<char>>) -> Self {
        Self {
            start_pos: find_coord_of_char_as_coord(&race_map, &'S').unwrap(),
            race_map,
            possible_cheats: vec![],
            time_util_coord: HashMap::new(),
        }
    }

    pub fn check_time_saved_per_cheat(&self) -> Vec<usize> {
        self.possible_cheats
            .iter()
            .map(|cheat| self.calculate_time_saved_by_cheat(cheat))
            .collect::<Vec<usize>>()
    }

    pub fn calculate_time_saved_by_cheat(&self, cheat: &(Coord, Coord)) -> usize {
        (self.time_util_coord.get(&cheat.1).unwrap() - 2)
            - self.time_util_coord.get(&cheat.0).unwrap()
    }

    pub fn check_paths_that_saves(&mut self) -> usize {
        let mut new_map = self.race_map.clone();
        let mut current_coord: Option<Coord> = Some(self.start_pos);
        let mut current_time: usize = 0;

        loop {
            if let Some(next_coord) = current_coord {
                next_coord.mark_coord_as_visited(&mut new_map);
                current_coord = self.check_for_cheats(&new_map, &next_coord);
                if self.time_util_coord.contains_key(&next_coord) {
                    self.time_util_coord
                        .entry(next_coord)
                        .and_modify(|f| *f += current_time);
                }
                current_time += 1;
            } else {
                break;
            }
        }

        self.possible_cheats.len()
    }

    fn check_for_cheats(
        &mut self,
        current_map: &Vec<Vec<char>>,
        current_coord: &Coord,
    ) -> Option<Coord> {
        let surroundings = current_coord.check_surroundings(&current_map, vec!['.', '#', 'E']);
        let mut next_coord: Option<Coord> = None;

        for (dir, coord) in surroundings {
            if coord.check_char_at(&current_map) == '#' {
                if let Some((coord, character)) = coord.check_next_coord_by_dir(&current_map, dir) {
                    if character == '.' || character == 'E' {
                        self.possible_cheats.push((*current_coord, coord));
                        self.time_util_coord.insert(*current_coord, 0);
                        self.time_util_coord.insert(coord, 0);
                    }
                }
            } else {
                if coord.check_char_at(&current_map) == 'E' {
                    self.time_util_coord.insert(coord, 0);
                }

                next_coord = Some(coord);
            }
        }

        next_coord
    }
}
