use aoc_utils::{
    check::{check_surroundings_char, MatrixPos},
    display::display_matrix,
    find::find_coord_of_char,
};

use crate::constants::Facing;

pub struct Mazev4 {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_facing: Facing,
    queue: Vec<((usize, usize), Facing, usize)>,
}

impl Mazev4 {
    pub fn new(map: &Vec<Vec<char>>, start_facing: Facing) -> Self {
        Self {
            start_pos: find_coord_of_char(&map, &'S').unwrap(),
            start_facing,
            map: map.clone(),
            queue: vec![],
        }
    }

    pub fn find_best_path(&mut self) -> Vec<usize> {
        self.queue.push((self.start_pos, self.start_facing, 0));

        let mut possible_results = vec![];

        loop {
            if self.queue.len() == 0 {
                break;
            }

            self.sort_queue_by_cost();

            let item = self.queue.remove(0);
            if let Some(val) = self.next_path(item.0, item.1, item.2) {
                possible_results.push(val);
            }
        }
        display_matrix(&self.map);

        println!("fastest: {}", possible_results.iter().min().unwrap());

        possible_results
    }

    fn next_path(
        &mut self,
        start_pos: (usize, usize),
        start_facing: Facing,
        start_cost: usize,
    ) -> Option<usize> {
        let start_cost = start_cost + 1;
        let mut result: Option<usize> = None;

        let surroundings = check_surroundings_char(
            &self.map,
            start_pos,
            vec!['.', 'E'],
            vec![Mazev4::map_matrix_pos_inverted(&start_facing)],
        );

        if surroundings.is_none() {
            return None;
        }

        let surroundings = surroundings.unwrap();
        if surroundings.len() == 1 {
            Mazev4::mark_as_visited(&mut self.map, &start_pos);
        }

        surroundings
            .iter()
            .for_each(|(matrix_pos, pos, character)| {
                if character == &'E' {
                    result = Some(start_cost);
                } else {
                    let new_facing = Mazev4::map_facing(matrix_pos);
                    let facing_cost = Mazev4::check_facing_cost(&start_facing, &new_facing);
                    self.queue
                        .push((*pos, new_facing, start_cost + facing_cost));
                }
            });

        return result;
    }

    fn sort_queue_by_cost(&mut self) {
        self.queue
            .sort_by(|(_, _, cost_a), (_, _, cost_b)| cost_a.cmp(cost_b));
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
