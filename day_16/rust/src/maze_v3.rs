use std::{collections::HashMap, thread, time::Duration};

use aoc_utils::{
    check::{check_surroundings_char, MatrixPos},
    display::display_matrix,
    find::find_coord_of_char,
};

use crate::constants::{CacheItem, Facing};

pub struct Mazev3<'a, 'b> {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_facing: Facing,
    cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItem>,
    count: &'b mut Option<usize>,
    total_count: usize,
}

impl<'a, 'b> Mazev3<'a, 'b> {
    pub fn new(
        map: &Vec<Vec<char>>,
        start_facing: Facing,
        cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItem>,
        count: &'b mut Option<usize>,
    ) -> Self {
        Self {
            start_pos: find_coord_of_char(&map, &'S').unwrap(),
            start_facing,
            map: map.clone(),
            cache,
            count,
            total_count: 0,
        }
    }

    pub fn new_child(
        map: &Vec<Vec<char>>,
        start_facing: Facing,
        start_pos: (usize, usize),
        cache: &'a mut HashMap<(Facing, (usize, usize)), CacheItem>,
        count: &'b mut Option<usize>,
        total_count: usize,
    ) -> Self {
        Self {
            start_pos,
            start_facing,
            map: map.clone(),
            cache,
            count,
            total_count,
        }
    }

    pub fn find_best_path(&mut self) -> Option<usize> {
        let mut current_pos = self.start_pos;
        let mut c_cost: usize = 0;
        let mut current_facing = self.start_facing;

        if let Some(cache_item) = self.cache.get(&(current_facing, current_pos)) {
            match cache_item {
                CacheItem::FINAL(cost) => {
                    println!("final: {}", self.total_count + c_cost + *cost);
                    return Some(*cost + c_cost);
                }
                CacheItem::WALL => return None,
                CacheItem::POS(next_pos, val) => {
                    current_pos = *next_pos;
                    c_cost += val;
                }
            }
        }

        if let Some(min) = self.count {
            if self.total_count > *min {
                return None;
            }
        }

        loop {
            // display_matrix(&self.map);
            // thread::sleep(Duration::from_millis(5));

            self.map[current_pos.1][current_pos.0] = 'X';

            let vec_new_pos = check_surroundings_char(
                &self.map,
                current_pos,
                vec!['.', 'E', 'X'],
                vec![Mazev3::map_matrix_pos_inverted(&current_facing)],
            );

            if let Some(vec_new_pos) = vec_new_pos {
                c_cost += 1;

                if vec_new_pos.len() == 1 {
                    if vec_new_pos[0].2 == 'E' {
                        println!("final: {}", self.total_count + c_cost);
                        self.cache.insert(
                            (current_facing, self.start_pos.clone()),
                            CacheItem::FINAL(c_cost),
                        );
                        if let Some(min) = self.count {
                            let n_t = self.total_count + c_cost;
                            if n_t < *min {
                                *self.count = Some(n_t);
                            }
                        } else {
                            *self.count = Some(self.total_count);
                        }
                        return Some(c_cost);
                    }

                    let new_facing = Mazev3::map_facing(&vec_new_pos[0].0);
                    if new_facing != current_facing {
                        c_cost += 1000;
                        current_facing = new_facing;
                    }
                    current_pos = vec_new_pos[0].1;
                    continue;
                }

                let mut vec = Vec::new();
                self.cache.insert(
                    (self.start_facing, self.start_pos.clone()),
                    CacheItem::POS(current_pos, c_cost - 1),
                );

                for elem in vec_new_pos {
                    if elem.2 != 'X' {
                        let new_facing = Mazev3::map_facing(&elem.0);
                        let to_add: usize;
                        if new_facing != current_facing {
                            to_add = 1000;
                        } else {
                            to_add = 0;
                        }
                        let mut m = self.map.clone();
                        m[elem.1 .1][elem.1 .0] = 'X';
                        let mut maze = Mazev3::new_child(
                            &m,
                            new_facing,
                            elem.1,
                            &mut self.cache,
                            &mut self.count,
                            self.total_count + c_cost - 1 + to_add,
                        );
                        if let Some(val) = maze.find_best_path() {
                            vec.push(val + to_add);
                        }
                    }
                }
                if !vec.is_empty() {
                    let res = vec.iter().map(|f| *f + c_cost).min();

                    return res;
                }
            }
            self.cache
                .insert((current_facing, self.start_pos.clone()), CacheItem::WALL);

            return None;
        }
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
