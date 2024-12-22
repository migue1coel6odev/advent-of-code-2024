use std::collections::HashSet;

use aoc_utils::{coord::Coord, display::display_matrix};

pub struct Maze<'a, 'b> {
    map: &'a Vec<Vec<char>>,
    start_pos: &'b Coord,
    end_char: Vec<char>,
    marker: char,
    path: Vec<char>,
}

impl<'a, 'b> Maze<'a, 'b> {
    pub fn new(
        map: &'a Vec<Vec<char>>,
        start_pos: &'b Coord,
        end_char: Vec<char>,
        path: Vec<char>,
        marker: char,
    ) -> Self {
        Self {
            map,
            start_pos,
            end_char,
            path,
            marker,
        }
    }

    pub fn find_all_paths(
        &self,
        max_paths: usize,
        max_depth: usize,
        vec_coord_to_add: Vec<Coord>,
    ) -> usize {
        let mut found_paths = 0;
        let mut queue: Vec<Vec<Coord>> = vec![vec![*self.start_pos]];
        let mut current_map = self.map.clone();
        let mut current_iteration = 0;
        let mut vec_coord_to_add = vec_coord_to_add.clone();
        let mut visited: Vec<Coord> = vec![];

        while queue.len() > 0 {
            let current_iteration_coords = queue.remove(0);

            let mut next_iteration_coords: HashSet<Coord> = HashSet::new();
            for coord in current_iteration_coords {
                found_paths += self.check_next_point(
                    &mut current_map,
                    coord,
                    &mut next_iteration_coords,
                    &mut visited,
                );
            }
            if next_iteration_coords.len() > 0 {
                queue.push(
                    next_iteration_coords
                        .iter()
                        .map(|f| *f)
                        .collect::<Vec<Coord>>(),
                );
            }

            current_iteration += 1;
            if found_paths == max_paths
                || current_iteration == max_depth
                || vec_coord_to_add.len() == 0
                || !self.check_is_possible(&current_map)
            {
                break;
            }

            self.walk_through_next_coord_in_iteration(&mut current_map, &mut vec_coord_to_add);
        }

        found_paths
    }

    fn check_next_point(
        &self,
        map: &mut Vec<Vec<char>>,
        coord: Coord,
        next_iteration_coords: &mut HashSet<Coord>,
        visited: &mut Vec<Coord>,
    ) -> usize {
        let mut ends_found = 0;
        let possible_next_coords = coord.check_surroundings_v2(&map);

        if possible_next_coords.len() == 0 {
            return 0;
        }

        for next_coord in possible_next_coords {
            if visited.contains(&next_coord) {
                continue;
            }
            let char_at_coord = next_coord.check_char_at(&map);
            visited.push(next_coord);
            if self.end_char.contains(&char_at_coord) {
                ends_found += 1;
            }

            if self.path.contains(&char_at_coord) {
                next_iteration_coords.insert(next_coord);
            }
        }

        ends_found
    }

    fn walk_through_next_coord_in_iteration(
        &self,
        mut map: &mut Vec<Vec<char>>,
        vec_coord_to_add: &mut Vec<Coord>,
    ) {
        self.mark_visited(&mut map, &vec_coord_to_add.remove(0));
    }

    fn check_is_possible(&self, current_map: &Vec<Vec<char>>) -> bool {
        current_map
            .iter()
            .find(|line| line.iter().filter(|c| self.end_char.contains(c)).count() > 0)
            .is_some()
    }

    fn mark_visited(&self, map: &mut Vec<Vec<char>>, point: &Coord) {
        map[point.y][point.x] = self.marker;
    }
}
