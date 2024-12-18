use std::{collections::HashMap, ops::Add, thread, time::Duration};

use aoc_utils::{
    check::{check_surroundings_char, check_surroundings_char_per_pos, MatrixPos},
    command::pause,
    display::display_matrix,
    find::find_coord_of_char,
};

use crate::constants::Facing;

pub struct Mazev2<'a, 'b> {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_facing: Facing,
    cache: &'a mut HashMap<(Facing, (usize, usize)), Option<usize>>,
    count: &'b mut Option<usize>,
    total_count: usize,
}

impl<'a, 'b> Mazev2<'a, 'b> {
    pub fn new(
        map: &Vec<Vec<char>>,
        start_facing: Facing,
        cache: &'a mut HashMap<(Facing, (usize, usize)), Option<usize>>,
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
        cache: &'a mut HashMap<(Facing, (usize, usize)), Option<usize>>,
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

    pub fn find_best_path(&mut self) -> (Option<usize>, bool) {
        let mut current_pos = self.start_pos;
        let mut c_cost: usize = 0;
        let mut current_facing = self.start_facing;
        let mut can_cache = true;

        loop {
            display_matrix(&self.map);
            // thread::sleep(Duration::from_millis(5));
            pause();

            self.map[current_pos.1][current_pos.0] = 'X';

            let vec_new_pos =
                check_surroundings_char(&self.map, current_pos, vec!['.', 'E', 'X'], vec![]);

            if let Some(vec_new_pos) = vec_new_pos {
                let filtered = vec_new_pos.iter().filter(|(_, _, c)| c == &'X').count();
                println!("filtered nr: {} {}", filtered, &vec_new_pos.len());

                c_cost += 1;
                self.total_count += 1;

                if let Some(min) = self.count {
                    if self.total_count > *min {
                        return (None, can_cache);
                    }
                }

                if vec_new_pos.len() - filtered == 1 {
                    let vnp: Vec<&(MatrixPos, (usize, usize), char)> = vec_new_pos
                        .iter()
                        .filter(|(_, _, c)| c != &'X')
                        .collect::<Vec<_>>();
                    if vnp[0].2 == 'E' {
                        // display_matrix(&self.map);
                        // println!("Cost: {}", c_cost);
                        // pause();
                        println!("total_count = {:?}", self.total_count);
                        *self.count = Some(self.total_count);
                        return (Some(c_cost), can_cache);
                    }

                    let new_facing = Mazev2::map_facing(&vnp[0].0);
                    if new_facing != current_facing {
                        c_cost += 1000;
                        self.total_count += 1000;
                        current_facing = new_facing;
                    }
                    current_pos = vnp[0].1;
                    continue;
                } else {
                    let mut vec = Vec::new();
                    if filtered > 1 {
                        can_cache = false;
                    }

                    for elem in vec_new_pos {
                        let new_facing = Mazev2::map_facing(&elem.0);
                        let to_add: usize;
                        if new_facing != current_facing {
                            to_add = 1000;
                        } else {
                            to_add = 0;
                        }
                        if let Some(val) = self.cache.get(&(new_facing, elem.1)) {
                            if let Some(val) = val {
                                vec.push(*val + to_add);
                            }
                        } else {
                            let mut m = self.map.clone();
                            m[elem.1 .1][elem.1 .0] = 'X';
                            let mut maze = Mazev2::new_child(
                                &m,
                                new_facing,
                                elem.1,
                                &mut self.cache,
                                &mut self.count,
                                self.total_count + to_add,
                            );
                            let (r, cc) = maze.find_best_path();
                            if let Some(val) = r {
                                if cc {
                                    self.cache.insert((new_facing, elem.1.clone()), Some(val));
                                }
                                vec.push(val + to_add);
                            } else {
                                if cc {
                                    self.cache.insert((new_facing, elem.1.clone()), None);
                                }
                            }
                        }
                    }
                    if vec.len() > 0 {
                        let res = vec.iter().map(|f| *f + c_cost).min();

                        return (res, can_cache);
                    }
                }
            }
            return (None, can_cache);
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
}
