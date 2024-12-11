use std::collections::HashMap;

pub struct MagicStonesFast {
    stones: Vec<usize>,
    current_blink: usize,
    predict: HashMap<usize, (usize, Vec<usize>)>,
}

impl MagicStonesFast {
    pub fn new(starting_stone: usize) -> Self {
        Self {
            stones: vec![starting_stone],
            current_blink: 0,
            predict: HashMap::new(),
        }
    }

    pub fn new_multiple(starting_stones: Vec<usize>) -> Self {
        Self {
            stones: starting_stones.clone(),
            current_blink: 0,
            predict: HashMap::new(),
        }
    }

    pub fn blink(&mut self, blink_nr: usize) -> Vec<usize> {
        loop {
            if self.current_blink == blink_nr {
                break;
            }
            self.current_blink += 1;
            self.stones = MagicStonesFast::_calculate_next_iteration(&self.stones);
        }

        self.stones.clone()
    }

    fn _calculate_next_iteration(stones: &Vec<usize>) -> Vec<usize> {
        let mut temp_vec = vec![];

        for &stone in stones {
            if stone == 0 {
                temp_vec.push(1);
                continue;
            }

            let nr_as_char_vec = stone.to_string().chars().collect::<Vec<char>>();
            if nr_as_char_vec.len() % 2 == 0 {
                let half = nr_as_char_vec.len() / 2;
                let pow = (10 as i32).pow(half as u32);
                temp_vec.push(stone / pow as usize);
                temp_vec.push(stone % pow as usize);
                continue;
            }
            temp_vec.push(stone * 2024);
        }

        temp_vec
    }
}
