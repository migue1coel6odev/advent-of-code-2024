use aoc_utils::display::display_matrix;

pub struct Warehouse {
    map: Vec<Vec<char>>,
    current_map: Vec<Vec<char>>,
    current_pos: (usize, usize),
}

pub enum Axis {
    X,
    Y,
}

impl Warehouse {
    pub fn new(map: &Vec<Vec<char>>) -> Self {
        Self {
            map: map.clone(),
            current_map: map.clone(),
            current_pos: map
                .iter()
                .enumerate()
                .find_map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .find_map(|(x, c)| if c == &'@' { Some((x, y)) } else { None })
                })
                .unwrap(),
        }
    }

    pub fn run_instructions(&mut self, instructions: Vec<char>) {
        for instruction in instructions {
            let next_pos = match instruction {
                '^' => self.move_to(self.current_pos, Axis::Y, -1, '@'),
                'v' => self.move_to(self.current_pos, Axis::Y, 1, '@'),
                '>' => self.move_to(self.current_pos, Axis::X, 1, '@'),
                '<' => self.move_to(self.current_pos, Axis::X, -1, '@'),
                _ => None,
            };

            if let Some(next_pos) = next_pos {
                self.current_pos = next_pos;
            }
        }

        display_matrix(&self.current_map);
        ()
    }

    pub fn move_to(
        &mut self,
        current_pos: (usize, usize),
        axis: Axis,
        val: isize,
        c: char,
    ) -> Option<(usize, usize)> {
        let next_pos = match axis {
            Axis::X => ((current_pos.0 as isize + val) as usize, current_pos.1),
            Axis::Y => (current_pos.0, (current_pos.1 as isize + val) as usize),
        };

        match self.current_map[next_pos.1][next_pos.0] {
            '.' => {
                self.current_map[current_pos.1][current_pos.0] = '.';
                self.current_map[next_pos.1][next_pos.0] = c;
                Some(next_pos)
            }
            'O' => {
                if self.move_to(next_pos, axis, val, 'O').is_some() {
                    self.current_map[current_pos.1][current_pos.0] = '.';
                    self.current_map[next_pos.1][next_pos.0] = c;
                    Some(next_pos)
                } else {
                    None
                }
            }
            '#' => None,
            _ => None,
        }
    }

    pub fn calculate_boxes_gps_coordinates(&self) -> usize {
        self.current_map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, c)| if c == &'O' { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum::<_>()
    }
}
