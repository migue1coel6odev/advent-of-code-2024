pub struct Warehouse {
    map: Vec<Vec<char>>,
    current_pos: (usize, usize),
}

impl Warehouse {
    pub fn new(map: &Vec<Vec<char>>) -> Self {
        Self {
            map: map.clone(),
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

    pub fn run_instructions(&self, instructions: Vec<char>) {}
}
