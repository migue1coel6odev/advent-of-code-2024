use std::collections::{HashMap, HashSet};

use aoc_utils::{
    check::{check_surroundings_char, MatrixPos},
    find::find_coord_of_char,
};

use crate::constants::Facing;

pub struct Maze2 {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_facing: Facing,
    queue: Vec<((usize, usize), Facing, usize, Vec<(usize, usize)>)>,
    common_positions: HashMap<(usize, usize), usize>,
}

impl Maze2 {
    pub fn new(map: &Vec<Vec<char>>, start_facing: Facing) -> Self {
        Self {
            start_pos: find_coord_of_char(&map, &'S').unwrap(),
            start_facing,
            map: map.clone(),
            queue: vec![],
            common_positions: HashMap::new(),
        }
    }

    pub fn find_best_path(&mut self) -> usize {
        self.queue
            .push((self.start_pos, self.start_facing, 0, vec![]));

        let mut possible_results = vec![];

        loop {
            if self.queue.len() == 0 {
                break;
            }

            self.sort_queue_by_cost();

            let item = self.queue.remove(0);
            if let Some(val) = self.next_path(item.0, item.1, item.2, item.3) {
                possible_results.push(val);
            }
        }

        let min = possible_results.iter().map(|(val, _)| *val).min().unwrap();

        let mut hashset: HashSet<(usize, usize)> = HashSet::new();

        possible_results
            .iter()
            .filter(|(val, _)| *val == min)
            .flat_map(|(_, vec)| vec.clone())
            .for_each(|pos| {
                hashset.insert(pos);
            });

        hashset.len()
    }

    fn next_path(
        &mut self,
        start_pos: (usize, usize),
        start_facing: Facing,
        start_cost: usize,
        previous_pos: Vec<(usize, usize)>,
    ) -> Option<(usize, Vec<(usize, usize)>)> {
        let start_cost = start_cost + 1;
        let mut result: Option<(usize, Vec<(usize, usize)>)> = None;

        let mut previous_pos = previous_pos.clone();
        previous_pos.push(start_pos);

        let surroundings = check_surroundings_char(
            &self.map,
            start_pos,
            vec!['.', 'E'],
            vec![Maze2::map_matrix_pos_inverted(&start_facing)],
        );

        if surroundings.is_none() {
            return None;
        }

        let surroundings = surroundings.unwrap();
        if surroundings.len() == 1 {
            Maze2::mark_as_visited(&mut self.map, &start_pos);
        } else {
            self.common_positions
                .entry(start_pos)
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }

        surroundings
            .iter()
            .for_each(|(matrix_pos, pos, character)| {
                if character == &'E' {
                    previous_pos.push(*pos);
                    result = Some((start_cost, previous_pos.clone()));
                } else {
                    let new_facing = Maze2::map_facing(matrix_pos);
                    let facing_cost = Maze2::check_facing_cost(&start_facing, &new_facing);
                    self.queue.push((
                        *pos,
                        new_facing,
                        start_cost + facing_cost,
                        previous_pos.clone(),
                    ));
                }
            });

        return result;
    }

    fn sort_queue_by_cost(&mut self) {
        self.queue
            .sort_by(|(_, _, cost_a, _), (_, _, cost_b, _)| cost_a.cmp(cost_b));
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
