use aoc_utils::display::display_matrix;

pub struct Warehousev2 {
    map: Vec<Vec<char>>,
    current_map: Vec<Vec<char>>,
    current_pos: (usize, usize),
}

pub enum Axis {
    X,
    Y,
}

impl Warehousev2 {
    pub fn new(map: &Vec<Vec<char>>) -> Self {
        let tweaked_map = map
            .iter()
            .map(|line| {
                line.iter()
                    .flat_map(|c| match &c {
                        '@' => vec!['@', '.'],
                        'O' => vec!['[', ']'],
                        _ => vec![*c; 2],
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<_>>>();

        Self {
            current_map: tweaked_map.clone(),
            current_pos: tweaked_map
                .iter()
                .enumerate()
                .find_map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .find_map(|(x, c)| if c == &'@' { Some((x, y)) } else { None })
                })
                .unwrap(),
            map: tweaked_map,
        }
    }

    pub fn run_instructions(&mut self, instructions: Vec<char>) {
        display_matrix(&self.current_map);
        for instruction in instructions {
            let next_pos = match instruction {
                '^' => self.move_to(self.current_pos, Axis::Y, -1, '@', true),
                'v' => self.move_to(self.current_pos, Axis::Y, 1, '@', true),
                '>' => self.move_to(self.current_pos, Axis::X, 1, '@', true),
                '<' => self.move_to(self.current_pos, Axis::X, -1, '@', true),
                _ => None,
            };

            if let Some(next_pos) = next_pos {
                self.current_pos = next_pos;
            }
        }

        ()
    }

    pub fn move_to(
        &mut self,
        current_pos: (usize, usize),
        axis: Axis,
        val: isize,
        c: char,
        write: bool,
    ) -> Option<(usize, usize)> {
        let next_pos = match axis {
            Axis::X => ((current_pos.0 as isize + val) as usize, current_pos.1),
            Axis::Y => (current_pos.0, (current_pos.1 as isize + val) as usize),
        };

        let next_char = self.current_map[next_pos.1][next_pos.0];
        let result = match (axis, next_char) {
            (Axis::Y, '[') => {
                let result;
                if self
                    .move_to(next_pos, Axis::Y, val, next_char, false)
                    .is_some()
                    && self
                        .move_to((next_pos.0 + 1, next_pos.1), Axis::Y, val, ']', false)
                        .is_some()
                {
                    self.move_to((next_pos.0 + 1, next_pos.1), Axis::Y, val, ']', write);
                    result = self.move_to(next_pos, Axis::Y, val, next_char, write)
                } else {
                    result = None
                }
                result
            }
            (Axis::Y, ']') => {
                let result;

                if self
                    .move_to(next_pos, Axis::Y, val, next_char, false)
                    .is_some()
                    && self
                        .move_to((next_pos.0 - 1, next_pos.1), Axis::Y, val, '[', false)
                        .is_some()
                {
                    self.move_to((next_pos.0 - 1, next_pos.1), Axis::Y, val, '[', write);
                    result = self.move_to(next_pos, Axis::Y, val, next_char, write);
                } else {
                    result = None;
                }
                result
            }
            (axis, c) => match c {
                '.' => Some(next_pos),
                '[' => self.move_to(next_pos, axis, val, c, true),
                ']' => self.move_to(next_pos, axis, val, c, true),
                '#' => None,
                _ => None,
            },
        };

        if result.is_some() {
            if write {
                self.current_map[current_pos.1][current_pos.0] = '.';
                self.current_map[next_pos.1][next_pos.0] = c;
            }
            return Some(next_pos);
        }

        None
    }

    pub fn calculate_boxes_gps_coordinates(&self) -> usize {
        self.current_map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, c)| if c == &'[' { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum::<_>()
    }
}
