pub struct MagicStones {
    stones: Vec<usize>,
    current_blink: usize,
}

impl MagicStones {
    pub fn new(starting_stone: usize) -> Self {
        Self {
            stones: vec![starting_stone],
            current_blink: 0,
        }
    }

    pub fn blink(&mut self, blink_nr: usize) -> Vec<usize> {
        loop {
            if self.current_blink == blink_nr {
                break;
            }
            self.current_blink += 1;
            self._calculate_next_iteration();
        }

        self.stones.clone()
    }

    fn _calculate_next_iteration(&mut self) {
        let mut temp_vec = vec![];

        for &stone in &self.stones {
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

        self.stones = temp_vec.clone();
    }
}
