#[derive(Debug)]
pub struct FindX_MAS {
    matrix: Vec<Vec<char>>,
    matrix_max_x: usize,
    matrix_max_y: usize,
    current_pos: (usize, usize),
    current_count: usize,
}

impl FindX_MAS {
    pub fn new(text: &Vec<String>) -> Self {
        let matrix = text
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        FindX_MAS {
            matrix_max_x: matrix.get(0).unwrap().len() - 1,
            matrix_max_y: matrix.len() - 1,
            matrix,
            current_pos: (0, 0),
            current_count: 0,
        }
    }

    pub fn find_xmas(&mut self) -> usize {
        loop {
            if !self.next_current_a() {
                break;
            }
            
            self.check_current_a();

        }

        self.current_count
    }

    fn check_current_a(&mut self) {

        if 
            self.check_top_left(self.current_pos, 'M').is_some() &&
            self.check_bottom_right(self.current_pos, 'S').is_some() &&
            self.check_top_right(self.current_pos, 'M').is_some() &&
            self.check_bottom_left(self.current_pos, 'S').is_some()
         {
            self.current_count += 1;
        }

        if 
            self.check_top_left(self.current_pos, 'M').is_some() &&
            self.check_bottom_right(self.current_pos, 'S').is_some() &&
            self.check_top_right(self.current_pos, 'S').is_some() &&
            self.check_bottom_left(self.current_pos, 'M').is_some()
         {
            self.current_count += 1;
        }

        if 
            self.check_top_left(self.current_pos, 'S').is_some() &&
            self.check_bottom_right(self.current_pos, 'M').is_some() &&
            self.check_top_right(self.current_pos, 'M').is_some() &&
            self.check_bottom_left(self.current_pos, 'S').is_some()
         {
            self.current_count += 1;
        }


        if 
            self.check_top_left(self.current_pos, 'S').is_some() &&
            self.check_bottom_right(self.current_pos, 'M').is_some() &&
            self.check_top_right(self.current_pos, 'S').is_some() &&
            self.check_bottom_left(self.current_pos, 'M').is_some()
         {
            self.current_count += 1;
        }

    }

    fn next_current_a(&mut self) -> bool {
        let mut start_x = self.current_pos.0 + 1;
        for y in self.current_pos.1..self.matrix_max_y + 1 {
            for x in start_x..self.matrix_max_x + 1 {
                if self.matrix.get(y).unwrap().get(x).unwrap() == &'A'
                    && x >= 1
                    && y >= 1
                    && x < self.matrix_max_x
                    && y < self.matrix_max_y
                {
                    self.current_pos = (x, y);
                    return true;
                }
            }
            start_x = 0;
        }
        false
    }

    fn check_top_left(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0 - 1;
        let y = position.1 - 1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }

    fn check_top_right(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0 + 1;
        let y = position.1 - 1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }

    fn check_bottom_left(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0 - 1;
        let y = position.1 + 1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }

    fn check_bottom_right(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0 + 1;
        let y = position.1 + 1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }
}
