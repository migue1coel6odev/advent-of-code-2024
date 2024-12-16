use crate::display_map::DisplayMap;

pub struct Plot {
    id: char,
    original_pos: Vec<(usize, usize)>,
    map: Vec<Vec<char>>,
}

impl Plot {
    pub fn new(id: char, original_pos: Vec<(usize, usize)>) -> Self {
        Self {
            id,
            map: Plot::build_map(&original_pos, id),
            original_pos,
        }
    }

    pub fn calculate_fencing(&self) -> usize {
        &self.calculate_area() * &self.calculate_perimeter()
    }

    pub fn calculate_fencing_sides(&self) -> usize {
        let area = &self.calculate_area();
        let sides = &self.calculate_sides();

        if self.id == 'V' {
            println!(
                "fencing for {} = {} where a = {} and s = {}",
                self.id,
                area * sides,
                area,
                sides
            );

            DisplayMap::display(&self.map);
        }

        area * sides
    }

    fn calculate_sides(&self) -> usize {
        let mut sides_count = 0;
        let mut arr = [None; 4];

        let mut left = 0;
        let mut right = 0;
        let mut top = 0;
        let mut bottom = 0;

        for (y, line) in self.map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == self.id {
                    let sides = Plot::check_sides(&self.map, &(x, y));

                    if self.id == 'V' {
                        println!("pos: {:?} arr: {:?}", (x, y), sides);
                    }

                    if let Some(left_side) = sides[0] {
                        if arr[0].is_none() || arr[0].is_some() && left_side != arr[0].unwrap() {
                            arr[0] = Some(left_side);
                            sides_count += 1;
                            left += 1;
                        }
                    }
                    if let Some(right_side) = sides[1] {
                        if arr[1].is_none() || arr[1].is_some() && right_side != arr[1].unwrap() {
                            arr[1] = Some(right_side);
                            sides_count += 1;
                            right += 1;
                        }
                    }
                    if let Some(top_side) = sides[2] {
                        if arr[2].is_none() || arr[2].is_some() && top_side != arr[2].unwrap() {
                            arr[2] = Some(top_side);
                            sides_count += 1;
                            top += 1;
                        }
                    } else {
                        arr[2] = None;
                    }
                    if let Some(bottom_side) = sides[3] {
                        if arr[3].is_none() || arr[3].is_some() && bottom_side != arr[3].unwrap() {
                            arr[3] = Some(bottom_side);
                            sides_count += 1;
                            bottom += 1;
                        }
                    } else {
                        arr[3] = None;
                    }
                }
            }
        }

        if self.id == 'V' {
            println!(
                "left: {} | right: {} | top: {} | bottom: {}",
                left, right, top, bottom
            );
        }

        sides_count
    }

    fn calculate_area(&self) -> usize {
        self.map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|c| if c != &'.' { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn calculate_perimeter(&self) -> usize {
        let mut perimeter = 0;
        for (y, line) in self.map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == self.id {
                    perimeter += Plot::check_surroundings(&self.map, &(x, y));
                }
            }
        }
        perimeter
    }

    fn check_sides(vec: &Vec<Vec<char>>, current_pos: &(usize, usize)) -> [Option<usize>; 4] {
        let mut arr = [None; 4];
        if current_pos.0 > 0 && vec[current_pos.1][current_pos.0 - 1] == '.' {
            arr[0] = Some(current_pos.0 - 1);
        }
        if current_pos.0 < vec[0].len() - 1 && vec[current_pos.1][current_pos.0 + 1] == '.' {
            arr[1] = Some(current_pos.0 + 1);
        }

        if current_pos.1 > 0 && vec[current_pos.1 - 1][current_pos.0] == '.' {
            arr[2] = Some(current_pos.1 - 1);
        }
        if current_pos.1 < vec.len() - 1 && vec[current_pos.1 + 1][current_pos.0] == '.' {
            arr[3] = Some(current_pos.1 + 1);
        }

        arr
    }

    fn check_surroundings(vec: &Vec<Vec<char>>, current_pos: &(usize, usize)) -> usize {
        let mut count = 0;
        if current_pos.0 > 0 && vec[current_pos.1][current_pos.0 - 1] == '.' {
            count += 1;
        }
        if current_pos.0 < vec[0].len() - 1 && vec[current_pos.1][current_pos.0 + 1] == '.' {
            count += 1;
        }

        if current_pos.1 > 0 && vec[current_pos.1 - 1][current_pos.0] == '.' {
            count += 1;
        }
        if current_pos.1 < vec.len() - 1 && vec[current_pos.1 + 1][current_pos.0] == '.' {
            count += 1;
        }

        count
    }

    fn build_map(positions: &Vec<(usize, usize)>, plot_id: char) -> Vec<Vec<char>> {
        let x_pos = positions.iter().map(|(x, _)| *x).collect::<Vec<_>>();
        let y_pos = positions.iter().map(|(_, y)| *y).collect::<Vec<_>>();

        let x_min = x_pos.iter().map(|f| *f).min().unwrap();
        let x_max = x_pos.iter().map(|f| *f).max().unwrap() - x_min;

        let y_min = y_pos.iter().map(|f| *f).min().unwrap();
        let y_max = y_pos.iter().map(|f| *f).max().unwrap() - y_min;

        let mut vec = vec![vec!['.'; x_max + 3]; y_max + 3];

        for pos in positions {
            vec[pos.1 - y_min + 1][pos.0 - x_min + 1] = plot_id;
        }

        vec
    }
}
