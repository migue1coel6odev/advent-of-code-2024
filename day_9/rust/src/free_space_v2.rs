use std::{cmp::Ordering, fmt::Display};

pub struct FreeSpaceV2 {
    input: String,
    disk_space: Vec<usize>,
    compacted_space: Vec<(usize, usize)>,
    compacted_space_v2: Vec<(Option<usize>, usize)>,
    checksum: usize,
}

impl FreeSpaceV2 {
    pub fn new(input: String) -> Self {
        FreeSpaceV2 {
            input,
            disk_space: Vec::new(),
            compacted_space: Vec::new(),
            compacted_space_v2: Vec::new(),
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

    pub fn compact_disk_space_v2(&mut self) {
        let mut current_index: usize = 0;

        let mut temp = self
            .disk_space
            .iter()
            .enumerate()
            .map(|(index, &size)| {
                if index == 0 || index % 2 == 0 {
                    let result = (Some(current_index), size);
                    current_index += 1;
                    return result;
                }
                return (None, size);
            })
            .collect::<Vec<(Option<usize>, usize)>>();

        current_index -= 1;
        loop {
            let (last_index, value) = temp
                .iter()
                .enumerate()
                .rev()
                .find(|(_, val)| {
                    if val.0.is_none() {
                        return false;
                    }
                    if let Some(indd) = val.0 {
                        if indd == current_index {
                            current_index -= 1;
                            return true;
                        }
                    }
                    return false;
                })
                .map(|(ind, f)| (ind, *f))
                .unwrap();

            let mut found = false;
            let mut skip = false;
            temp = temp
                .iter()
                .flat_map(|val| {
                    if val.0.is_none() && val.1 >= value.1 && found == false && skip == false {
                        found = true;
                        return vec![(value.0, value.1), (None, val.1 - value.1)];
                    }

                    if let Some(v) = val.0 {
                        if v == value.0.unwrap() {
                            skip = true;
                        }
                    }

                    return vec![*val];
                })
                .collect();

            if found {
                temp[last_index + 1] = (None, value.1);
            }

            if current_index == 1 {
                break;
            }
            // println!(
            //     "-> {:?}",
            //     temp.iter()
            //         .flat_map(|val| {
            //             if val.0.is_none() {
            //                 return vec!['.'.to_string(); val.1];
            //             }
            //             let value = val.0.unwrap();

            //             println!(" error: {}", value);
            //             return vec![value.to_string(); val.1];
            //         })
            //         .collect::<Vec<String>>()
            //         .join("")
            // );
        }
        self.compacted_space_v2 = temp.clone();
    }

    pub fn update_filesystem_checksum(&mut self) {
        self.checksum = self
            .compacted_space_v2
            .iter()
            .flat_map(|(index, size)| vec![index; *size])
            .enumerate()
            .map(|(index, digit)| {
                if let Some(nr) = digit {
                    return index * nr;
                }
                return 0;
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
        // text.push_str(
        //     format!(
        //         "\n\n| COMPACTED DISK SPACE |\n{} ",
        //         &self
        //             .compacted_space
        //             .iter()
        //             .map(|(index, size)| vec![index; *size]
        //                 .iter()
        //                 .map(|f| f.to_string())
        //                 .collect::<Vec<_>>()
        //                 .join(""))
        //             .collect::<Vec<_>>()
        //             .join("")
        //     )
        //     .as_str(),
        // );
        // text.push_str(
        //     format!(
        //         "\n\n| COMPACTED DISK SPACE |\n{} ",
        //         &self
        //             .compacted_space_v2
        //             .iter()
        //             .map(|(index, size)| vec![index; *size]
        //                 .iter()
        //                 .map(|f| match f {
        //                     Some(x) => x.to_string(),
        //                     None => ".".to_string(),
        //                 })
        //                 .collect::<Vec<_>>()
        //                 .join(""))
        //             .collect::<Vec<_>>()
        //             .join("")
        //     )
        //     .as_str(),
        // );
        text.push_str(format!("\n\n| FILESYSTEM CHECKSUM |\n{} ", &self.checksum).as_str());

        write!(f, "{}", text)
    }
}
