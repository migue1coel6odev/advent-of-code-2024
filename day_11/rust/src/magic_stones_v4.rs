use std::collections::HashMap;

pub struct MagicStonesSuperSuperFast {
    stones: HashMap<usize, usize>,
}

impl MagicStonesSuperSuperFast {
    pub fn new(stones: HashMap<usize, usize>) -> Self {
        Self { stones }
    }

    pub fn blink(&mut self, blink_nr: usize) -> usize {
        let mut current_blink = 0;

        loop {
            if current_blink == blink_nr {
                break;
            }

            let mut temp_hash = HashMap::new();

            for (stone, count) in &self.stones {
                let res = MagicStonesSuperSuperFast::_calculate_next_iteration(*stone);
                for val in res {
                    let entry = temp_hash.entry(val).or_insert(0);
                    *entry += *count;
                }
            }

            self.stones = temp_hash.clone();

            current_blink += 1;
        }

        self.stones.iter().map(|(_, value)| *value).sum()
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
