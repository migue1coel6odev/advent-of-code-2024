use std::collections::HashMap;

pub struct MagicStonesFast {
    stones: Vec<usize>,
    predict: HashMap<usize, (usize, Vec<usize>)>,
    predict_depth: usize,
}

impl MagicStonesFast {
    pub fn new(starting_stones: Vec<usize>, predict_depth: usize) -> Self {
        Self {
            stones: starting_stones.clone(),
            predict: HashMap::new(),
            predict_depth,
        }
    }

    pub fn blink(&mut self, blink_nr: usize) -> Vec<usize> {
        let mut result = vec![];
        let mut current_blink = blink_nr;
        'outer: loop {
            if current_blink % 5 == 0 {
                let mut index = 0;
                let max_index = self.stones.len() - 1;
                loop {
                    if index > max_index {
                        break 'outer;
                    }

                    let stone = self.stones[index];

                    result.push(self._predict(stone, current_blink));

                    println!("index: {} len : {}", index, result.len());

                    index += 1;
                }
            } else {
                loop {
                    if current_blink % 5 == 0 {
                        break;
                    }
                    self.stones = MagicStonesFast::_calculate_next_iteration_mul(&self.stones);
                    current_blink -= 1;
                }
            }
        }

        result.iter().flat_map(|f| f.clone()).collect::<Vec<_>>()
    }

    fn _predict(&mut self, stone: usize, depth: usize) -> Vec<usize> {
        if let Some((st, vec)) = self.predict.get(&depth) {
            if st == &stone {
                // print!(" {:?}", self.predict);
                return vec.clone();
            }
        }

        if depth == 5 {
            let mut temp = vec![stone];
            (0..5).for_each(|_| {
                temp = MagicStonesFast::_calculate_next_iteration_mul(&temp);
            });
            self.predict.insert(depth, (stone, temp.clone()));
            return temp;
        }

        let mut result = vec![];
        let res = self._predict(stone, depth - 5);
        for st in res {
            let r = self._predict(st, depth - 5);
            if depth <= 15 {
                self.predict.insert(depth, (stone, r.clone()));
            }
            result.push(r);
            // println!("iteration: {}")
        }

        let result = result
            .iter()
            .flat_map(|f| f.clone())
            .collect::<Vec<usize>>();

        result
    }

    fn _calculate_next_iteration_mul(stones: &Vec<usize>) -> Vec<usize> {
        let mut temp_vec = vec![];

        for &stone in stones {
            temp_vec.push(MagicStonesFast::_calculate_next_iteration(stone));
        }

        temp_vec.iter().flat_map(|f| f.clone()).collect()
    }

    fn _calculate_next_iteration(stone: usize) -> Vec<usize> {
        let mut temp_vec = vec![];

        if stone == 0 {
            temp_vec.push(1);
            return temp_vec;
        }

        let nr_as_char_vec = stone.to_string().chars().collect::<Vec<char>>();
        if nr_as_char_vec.len() % 2 == 0 {
            let half = nr_as_char_vec.len() / 2;
            let pow = (10 as i32).pow(half as u32);
            temp_vec.push(stone / pow as usize);
            temp_vec.push(stone % pow as usize);
            return temp_vec;
        }

        temp_vec.push(stone * 2024);

        temp_vec
    }
}
