use std::fmt::Display;

pub struct FreeSpaceV1 {
    input: String,
    disk_space: Vec<char>,
    compacted_space: Vec<char>,
    checksum: usize,
}

impl FreeSpaceV1 {
    pub fn new(input: String) -> Self {
        FreeSpaceV1 {
            input,
            disk_space: Vec::new(),
            compacted_space: Vec::new(),
            checksum: 0,
        }
    }

    pub fn decode_disk_space(&mut self) {
        let mut current_index = 0;
        for (index, size) in self.input.char_indices() {
            let mut char = '.';
            if index == 0 || index % 2 == 0 {
                char = char::from_digit(current_index as u32, 10).unwrap();
                current_index += 1;
            }
            self.disk_space = [
                self.disk_space.clone(),
                vec![char; size.to_digit(10).unwrap() as usize],
            ]
            .concat();
        }
    }

    pub fn compact_disk_space(&mut self) {
        self.compacted_space = self.disk_space.clone();
        let mut index = 0;
        let mut reverse_index = self.compacted_space.len() - 1;
        'outer: loop {
            if self.compacted_space.get(index).unwrap() == &'.' {
                loop {
                    let last = self.compacted_space[reverse_index];

                    if last != '.' {
                        self.compacted_space[index] = last;
                        self.compacted_space[reverse_index] = '.';
                        reverse_index -= 1;
                        break;
                    }
                    reverse_index -= 1;
                    if reverse_index <= index {
                        break 'outer;
                    }
                }
            }
            index += 1;
            if reverse_index <= index {
                break 'outer;
            }
        }
    }

    pub fn update_filesystem_checksum(&mut self) {
        self.checksum = self
            .compacted_space
            .iter()
            .enumerate()
            .map(|(index, digit)| {
                if digit == &'.' {
                    return 0;
                }

                return digit.to_digit(10).unwrap() as usize * index;
            })
            .sum()
    }
}

impl Display for FreeSpaceV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = format!("| ORIGINAL |\n{} ", &self.input);
        text.push_str(
            format!(
                "\n\n| DECODE DISK SPACE |\n{} ",
                &self
                    .disk_space
                    .iter()
                    .map(|char| char.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            )
            .as_str(),
        );
        text.push_str(
            format!(
                "\n\n| COMPACTED DISK SPACE |\n{} ",
                &self
                    .compacted_space
                    .iter()
                    .map(|char| char.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            )
            .as_str(),
        );
        text.push_str(format!("\n\n| FILESYSTEM CHECKSUM |\n{} ", &self.checksum).as_str());

        write!(f, "{}", text)
    }
}
