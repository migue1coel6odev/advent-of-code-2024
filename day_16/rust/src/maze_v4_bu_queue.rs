use std::{cmp::Ordering, collections::HashMap, thread, time::Duration};

use aoc_utils::{
    check::{
        check_distance_between_points, check_euclidian_distance_between_points,
        check_surroundings_char, MatrixPos,
    },
    command::pause,
    display::display_matrix,
    find::find_coord_of_char,
};

use crate::constants::{CacheItemV2, Facing};

pub struct Mazev4<'a, 'b> {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    start_facing: Facing,
    cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItemV2>,
    current_fastest: &'b mut Option<usize>,
    queue: Vec<(Vec<Vec<char>>, (usize, usize), Facing, usize)>,
}

impl<'a, 'b> Mazev4<'a, 'b> {
    pub fn new(
        map: &Vec<Vec<char>>,
        start_facing: Facing,
        cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItemV2>,
        current_fastest: &'b mut Option<usize>,
    ) -> Self {
        Self {
            start_pos: find_coord_of_char(&map, &'S').unwrap(),
            end_pos: find_coord_of_char(&map, &'E').unwrap(),
            start_facing,
            map: map.clone(),
            cache,
            current_fastest,
            queue: vec![],
        }
    }

    pub fn find_best_path(&mut self) -> Vec<usize> {
        self.queue
            .push((self.map.clone(), self.start_pos, self.start_facing, 0));

        let mut current_iter = 0;
        loop {
            println!("queue size: {}", self.queue.len());
            if self.queue.len() == 0 {
                break;
            }

            if let item = self.queue.remove(0) {
                self.next_path(item.0.clone(), item.1, item.2, item.3);
            }
            if current_iter == 100 {
                self.sort_queue_by_closer_to_end();
                current_iter = 0;
            } else {
                current_iter += 1;
            }
        }

        if let Some(current_fastest) = self.current_fastest {
            println!("fastest: {}", current_fastest);
        }

        vec![]
    }

    fn next_path(
        &mut self,
        start_map: Vec<Vec<char>>,
        start_pos: (usize, usize),
        start_facing: Facing,
        start_cost: usize,
    ) {
        let mut current_pos = start_pos;
        let mut current_facing = start_facing;
        let mut current_cost = start_cost;
        let mut current_map = start_map;

        if let Some(cache_item) = self.cache.get(&(current_facing, current_pos)) {
            match cache_item {
                CacheItemV2::FINAL(cost) => {
                    let run_value = current_cost + *cost;

                    self.check_current_fastest(run_value);

                    display_matrix(&current_map);
                }
                CacheItemV2::WALL => return (),
                CacheItemV2::POS(facing, next_pos, val) => {
                    current_facing = *facing;
                    current_pos = *next_pos;
                    current_cost += val;
                }
            }
        }

        loop {
            // display_matrix(&current_map);
            // thread::sleep(Duration::from_millis(5));
            // pause();

            if self.check_is_higher_current_fastest(current_cost)
                || self.check_cant_be_fastest(&current_pos, &current_cost)
            {
                return ();
            }

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
                return ();
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
                    self.cache.insert(
                        (start_facing, start_pos),
                        CacheItemV2::FINAL(current_cost - start_cost),
                    );

                    display_matrix(&current_map);

                    self.check_current_fastest(current_cost);

                    return ();
                }
                continue;
            }

            self.cache.insert(
                (start_facing, start_pos),
                CacheItemV2::POS(current_facing, current_pos, current_cost - start_cost),
            );
            self.handle_multiple_choices(&surroundings, &current_map, current_facing, current_cost);
            break;
        }
    }

    fn handle_multiple_choices(
        &mut self,
        elems: &Vec<(MatrixPos, (usize, usize), char)>,
        current_map: &Vec<Vec<char>>,
        facing: Facing,
        cost: usize,
    ) {
        self.sort_by_closer_to_end(elems)
            .iter()
            .for_each(|(start_facing, start_pos, character)| {
                if character != &'X' {
                    let mut new_map = current_map.clone();
                    Mazev4::mark_as_visited(&mut new_map, start_pos);
                    self.queue.push((
                        new_map,
                        *start_pos,
                        *start_facing,
                        cost + Mazev4::check_facing_cost(&facing, &start_facing) + 1,
                    ));
                }
            })
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

    fn sort_by_closer_to_end(
        &self,
        vec: &Vec<(MatrixPos, (usize, usize), char)>,
    ) -> Vec<(Facing, (usize, usize), char)> {
        let mut x = vec
            .iter()
            .map(|(matrix_pos, pos, character)| (Mazev4::map_facing(matrix_pos), *pos, *character))
            .collect::<Vec<(Facing, (usize, usize), char)>>();
        x.sort_by(|(_, pos_a, _), (_, pos_b, _)| {
            if check_euclidian_distance_between_points(&self.end_pos, &pos_a)
                > check_euclidian_distance_between_points(&self.end_pos, &pos_b)
            {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        x
    }

    fn sort_queue_by_closer_to_end(&mut self) {
        // let mut x = vec
        //     .iter()
        //     .map(|(matrix_pos, pos, character)| (Mazev4::map_facing(matrix_pos), *pos, *character))
        //     .collect::<Vec<(Facing, (usize, usize), char)>>();
        self.queue.sort_by(|(_, pos_a, _, _), (_, pos_b, _, _)| {
            if check_euclidian_distance_between_points(&self.end_pos, &pos_a)
                > check_euclidian_distance_between_points(&self.end_pos, &pos_b)
            {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }

    fn check_is_higher_current_fastest(&self, current_cost: usize) -> bool {
        if let Some(current_fastest) = &self.current_fastest {
            if current_cost > *current_fastest {
                return true;
            }
        }
        false
    }

    fn check_cant_be_fastest(&self, current_pos: &(usize, usize), current_cost: &usize) -> bool {
        if let Some(current_fastest) = &self.current_fastest {
            let val = check_distance_between_points(&self.end_pos, current_pos);
            if current_cost + val > *current_fastest {
                return true;
            }
        }
        false
    }

    fn check_current_fastest(&mut self, run_value: usize) {
        if let Some(current_fastest) = &self.current_fastest {
            if run_value < *current_fastest {
                println!("fastest so far: {}", run_value);
                pause();
                *self.current_fastest = Some(run_value);
            }
        } else {
            *self.current_fastest = Some(run_value);
        }
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
