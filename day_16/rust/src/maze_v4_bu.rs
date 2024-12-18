use std::collections::HashMap;

use aoc_utils::{
    check::{check_surroundings_char, MatrixPos},
    command::pause,
    display::display_matrix,
    find::find_coord_of_char,
};

use crate::constants::{CacheItemV2, Facing};

pub struct Mazev4<'a> {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_facing: Facing,
    cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItemV2>,
}

impl<'a> Mazev4<'a> {
    pub fn new(
        map: &Vec<Vec<char>>,
        start_facing: Facing,
        cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItemV2>,
    ) -> Self {
        Self {
            start_pos: find_coord_of_char(&map, &'S').unwrap(),
            start_facing,
            map: map.clone(),
            cache,
        }
    }

    pub fn find_best_path(&mut self) -> Vec<usize> {
        if let Some(results) =
            self.next_path(self.map.clone(), self.start_pos, self.start_facing, 0)
        {
            return results;
        }
        vec![]
    }

    fn next_path(
        &mut self,
        start_map: Vec<Vec<char>>,
        start_pos: (usize, usize),
        start_facing: Facing,
        start_cost: usize,
    ) -> Option<Vec<usize>> {
        let mut current_pos = start_pos;
        let mut current_facing = start_facing;
        let mut current_cost = start_cost;
        let mut current_map = start_map;

        loop {
            // display_matrix(&current_map);
            // println!("current_cost = {}", current_cost);
            // pause();

            Mazev4::mark_as_visited(&mut current_map, &current_pos);
            let surroundings = check_surroundings_char(
                &current_map,
                current_pos,
                vec!['.', 'E', 'X'],
                vec![Mazev4::map_matrix_pos_inverted(&current_facing)],
            );

            if surroundings.is_none() {
                self.cache
                    .insert((start_facing, start_pos), CacheItemV2::WALL);
                return None;
            }

            let surroundings = surroundings.unwrap();

            if surroundings.len() == 1 {
                let elem = surroundings.get(0).unwrap();
                if self
                    .handle_one_choice(
                        elem,
                        &mut current_pos,
                        &mut current_facing,
                        &mut current_cost,
                    )
                    .is_some()
                {
                    self.cache
                        .insert((start_facing, start_pos), CacheItemV2::FINAL(current_cost));
                    return Some(vec![current_cost]);
                }
                continue;
            }

            self.cache.insert(
                (start_facing, start_pos),
                CacheItemV2::POS(current_facing, current_pos, current_cost),
            );
            let results = self.handle_multiple_choices(
                &surroundings,
                &current_map,
                current_facing,
                current_cost,
            );
            if !results.is_empty() {
                return Some(results);
            }
            return None;
        }
    }

    fn handle_multiple_choices(
        &mut self,
        elems: &Vec<(MatrixPos, (usize, usize), char)>,
        current_map: &Vec<Vec<char>>,
        facing: Facing,
        cost: usize,
    ) -> Vec<usize> {
        elems
            .iter()
            .map(|(start_facing, start_pos, character)| {
                if character == &'X' {
                    return None;
                }
                let start_facing = Mazev4::map_facing(start_facing);
                let mut new_map = current_map.clone();
                Mazev4::mark_as_visited(&mut new_map, start_pos);
                self.next_path(
                    new_map,
                    *start_pos,
                    start_facing,
                    cost + Mazev4::check_facing_cost(&facing, &start_facing) + 1,
                )
            })
            .filter(|f| f.is_some())
            .flat_map(|f| f.unwrap())
            .collect::<Vec<usize>>()
    }

    fn handle_one_choice(
        &mut self,
        elem: &(MatrixPos, (usize, usize), char),
        pos: &mut (usize, usize),
        facing: &mut Facing,
        cost: &mut usize,
    ) -> Option<()> {
        let (next_facing, next_pos, character) = elem;
        let next_facing = Mazev4::map_facing(next_facing);
        *cost += 1;
        *cost += Mazev4::check_facing_cost(facing, &next_facing);
        *pos = *next_pos;
        *facing = next_facing;

        if character == &'E' {
            return Some(());
        }

        None
    }

    fn check_facing_cost(current_facing: &Facing, next_facing: &Facing) -> usize {
        if current_facing == next_facing {
            0
        } else {
            1000
        }
    }

    fn mark_as_visited(map: &mut Vec<Vec<char>>, pos: &(usize, usize)) {
        map[pos.1][pos.0] = 'X';
    }

    fn map_facing(matrix_pos: &MatrixPos) -> Facing {
        match matrix_pos {
            MatrixPos::UP => Facing::NORTH,
            MatrixPos::DOWN => Facing::SOUTH,
            MatrixPos::LEFT => Facing::WEST,
            MatrixPos::RIGHT => Facing::EAST,
        }
    }

    fn map_matrix_pos_inverted(matrix_pos: &Facing) -> MatrixPos {
        match matrix_pos {
            Facing::NORTH => MatrixPos::DOWN,
            Facing::SOUTH => MatrixPos::UP,
            Facing::WEST => MatrixPos::RIGHT,
            Facing::EAST => MatrixPos::LEFT,
        }
    }
}
