use std::fmt::Display;

pub struct Antenna {
    frequency: char,
    positions: Vec<(usize, usize)>,
    antinodes: Vec<(usize, usize)>,
    x_max_index: usize,
    y_max_index: usize,
}

impl Antenna {
    pub fn new(frequency: char, x_max_index: usize, y_max_index: usize) -> Self {
        Antenna {
            frequency,
            positions: Vec::new(),
            antinodes: Vec::new(),
            x_max_index,
            y_max_index,
        }
    }

    pub fn add_antenna_pos(&mut self, pos: (usize, usize)) {
        let _ = self.positions.push(pos);
    }

    pub fn generate_antinodes(&mut self) -> &Vec<(usize, usize)> {
        self.positions
            .iter()
            .enumerate()
            .for_each(|(index_a, antenna_a_pos)| {
                self.positions
                    .iter()
                    .enumerate()
                    .for_each(|(index_b, antenna_b_pos)| {
                        if index_a != index_b {
                            let antinode_pos = Antenna::_get_antinode(antenna_a_pos, antenna_b_pos);

                            if let Some((x, y)) = antinode_pos {
                                if x < self.x_max_index && y < self.y_max_index {
                                    self.antinodes.push((x, y));
                                }
                            }
                        }
                    });
            });

        return &self.antinodes;
    }

    fn _get_antinode(pos_a: &(usize, usize), pos_b: &(usize, usize)) -> Option<(usize, usize)> {
        let x;
        if pos_a.0 < pos_b.0 {
            x = pos_a.0.checked_sub(pos_a.0.abs_diff(pos_b.0));
        } else {
            x = pos_a.0.checked_add(pos_a.0.abs_diff(pos_b.0))
        }

        let y;
        if pos_a.1 < pos_b.1 {
            y = pos_a.1.checked_sub(pos_a.1.abs_diff(pos_b.1));
        } else {
            y = pos_a.1.checked_add(pos_a.1.abs_diff(pos_b.1))
        }

        if x.is_some() && y.is_some() {
            return Some((x.unwrap(), y.unwrap()));
        }

        return None;
    }
}

impl Display for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = format!("Frequency: {}\n", self.frequency);

        for y in 0..self.y_max_index {
            for x in 0..self.x_max_index {
                if self
                    .positions
                    .iter()
                    .find(|pos| pos.0 == x && pos.1 == y)
                    .is_some()
                {
                    text.push_str(format!("{}", self.frequency).as_str());
                    continue;
                }
                if self
                    .antinodes
                    .iter()
                    .find(|pos| pos.0 == x && pos.1 == y)
                    .is_some()
                {
                    text.push_str("#");
                    continue;
                }
                text.push_str(".");
            }
            text.push_str("\n");
        }

        write!(f, "{}", text)
    }
}
