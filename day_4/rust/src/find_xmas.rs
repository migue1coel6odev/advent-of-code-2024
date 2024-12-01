#[derive(Debug)]
pub struct FindXMAS {
    matrix: Vec<Vec<char>>,
    matrix_max_x: usize,
    matrix_max_y: usize,
    current_pos: (usize, usize),
    current_count: usize,
}

impl FindXMAS {
    pub fn new(text: &Vec<String>) -> Self {
        let matrix = text
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        let mut x_index = 0;
        let mut y_index = 0;

        matrix.iter().enumerate().find(|(index, line_chars)| {
            line_chars.iter().enumerate().find(|(line_char_index, &c)| {
                if c == 'X' {
                    x_index = *line_char_index;
                    return true;
                }

                return false;
            });

            y_index = *index;

            return true;
        });

        FindXMAS {
            matrix_max_x: matrix.get(0).unwrap().len() - 1,
            matrix_max_y: matrix.len() - 1,
            matrix,
            current_pos: (x_index, y_index),
            current_count: 0,
        }
    }

    pub fn find_xmas(&mut self) -> usize {
        loop {
            self.check_current_x();

            if !self.next_current_x() {
                break;
            }
        }

        self.current_count
    }

    fn check_current_x(&mut self) {
        // Check top left
        if self.current_pos.0 >= 3 && self.current_pos.1 >= 3 {
            match &self.check_top_left((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_top_left(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_top_left(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }

        // Check top right
        if self.current_pos.0 <= self.matrix_max_x - 3 && self.current_pos.1 >= 3 {
            match &self.check_top_right((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_top_right(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_top_right(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }

        // Check bottom left
        if self.current_pos.0 >= 3 && self.current_pos.1 <= self.matrix_max_y - 3 {
            match &self.check_bottom_left((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_bottom_left(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_bottom_left(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }

        // Check bottom right
        if self.current_pos.0 <= self.matrix_max_x - 3
            && self.current_pos.1 <= self.matrix_max_y - 3
        {
            match &self.check_bottom_right((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_bottom_right(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_bottom_right(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }

        // Check above
        if self.current_pos.1 >= 3 {
            match &self.check_above((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_above(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_above(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }

        // Check below
        if self.current_pos.1 <= self.matrix_max_y - 3 {
            match &self.check_below((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_below(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_below(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }
        // Check right
        if self.current_pos.0 <= self.matrix_max_x - 3 {
            match &self.check_right((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_right(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_right(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }
        // Check left
        if self.current_pos.0 >= 3 {
            match &self.check_left((self.current_pos.0, self.current_pos.1), 'M') {
                Some(new_pos) => match &self.check_left(*new_pos, 'A') {
                    Some(new_pos) => match &self.check_left(*new_pos, 'S') {
                        Some(_) => self.current_count += 1,
                        _ => (),
                    },
                    _ => (),
                },
                None => (),
            }
        }

        // if self.current_pos.0 >= 3 && self.current_pos.1 >= 3 {
        //     self.find_word(self.check_top_left);
        // }
    }

    // fn find_word(
    //     &mut self,
    //     f: fn(position: (usize, usize), letter: char) -> Option<(usize, usize)>,
    // ) {
    //     match f((self.current_pos.0, self.current_pos.1), 'M') {
    //         Some((new_x, new_y)) => match f((new_x, new_y), 'A') {
    //             Some((new_x, new_y)) => match f((new_x, new_y), 'S') {
    //                 Some(_) => self.current_count += 1,
    //                 _ => println!("No S"),
    //             },
    //             _ => println!("No A"),
    //         },
    //         None => println!("No M"),
    //     }
    // }

    fn next_current_x(&mut self) -> bool {
        let mut start_x = self.current_pos.0 + 1;
        for y in self.current_pos.1..self.matrix_max_y+1 {
            for x in start_x..self.matrix_max_x+1 {
                if self.matrix.get(y).unwrap().get(x).unwrap() == &'X' {
                    self.current_pos = (x, y);
                    return true;
                }
            }
            start_x = 0;
        }
        false
    }

    fn check_above(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0;
        let y = position.1 - 1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }

    fn check_below(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0;
        let y = position.1 + 1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }

    fn check_right(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0 + 1;
        let y = position.1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
    }

    fn check_left(&self, position: (usize, usize), letter: char) -> Option<(usize, usize)> {
        let x = position.0 - 1;
        let y = position.1;
        if &self.matrix.get(y).unwrap().get(x).unwrap() == &&letter {
            return Some((x, y));
        }
        return None;
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
