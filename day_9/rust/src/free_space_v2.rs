use std::{cmp::Ordering, fmt::Display};

pub struct FreeSpaceV2 {
    input: String,
    disk_space: Vec<usize>,
    compacted_space: Vec<(usize, usize)>,
    checksum: usize,
}

impl FreeSpaceV2 {
    pub fn new(input: String) -> Self {
        FreeSpaceV2 {
            input,
            disk_space: Vec::new(),
            compacted_space: Vec::new(),
            checksum: 0,
        }
    }

    pub fn decode_disk_space(&mut self) {
        for size in self.input.chars() {
            self.disk_space.push(size.to_digit(10).unwrap() as usize);
        }
    }

    pub fn compact_disk_space(&mut self) {
        let mut dup_decode = self
            .disk_space
            .iter()
            .step_by(2)
            .map(|v| *v)
            .collect::<Vec<_>>();

        let mut max_compressed_space: usize = self.disk_space.iter().step_by(2).map(|c| *c).sum();

        let mut current_index = 0;
        let mut reverse_index;
        'outer: for (index, space) in self.disk_space.iter().enumerate() {
            if max_compressed_space == 0 {
                break 'outer;
            }
            if index == 0 || index % 2 == 0 {
                if *space < max_compressed_space {
                    self.compacted_space.push((current_index, *space));
                    max_compressed_space -= *space;
                } else {
                    self.compacted_space
                        .push((current_index, max_compressed_space));
                    max_compressed_space = 0;
                }
                current_index += 1;
                continue;
            }

            let mut temp_space = *space;
            loop {
                reverse_index = dup_decode.len() - 1;
                let last_space = dup_decode[reverse_index];
                match temp_space.cmp(&last_space) {
                    Ordering::Equal => {
                        self.compacted_space.push((reverse_index, last_space));
                        max_compressed_space -= last_space;
                        dup_decode.remove(reverse_index);
                        break;
                    }
                    Ordering::Less => {
                        self.compacted_space.push((reverse_index, temp_space));
                        max_compressed_space -= temp_space;
                        dup_decode[reverse_index] -= temp_space;
                        break;
                    }
                    Ordering::Greater => {
                        self.compacted_space.push((reverse_index, last_space));
                        max_compressed_space -= last_space;
                        dup_decode.remove(reverse_index);
                        temp_space -= last_space;
                    }
                }
            }
        }
    }

    pub fn update_filesystem_checksum(&mut self) {
        self.checksum = self
            .compacted_space
            .iter()
            .flat_map(|(index, size)| vec![index; *size])
            .enumerate()
            .map(|(index, digit)| {
                return digit * index;
            })
            .sum()
    }
}

impl Display for FreeSpaceV2 {
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
                    .map(|(index, size)| vec![index; *size]
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(""))
                    .collect::<Vec<_>>()
                    .join("")
            )
            .as_str(),
        );
        text.push_str(format!("\n\n| FILESYSTEM CHECKSUM |\n{} ", &self.checksum).as_str());

        write!(f, "{}", text)
    }
}
