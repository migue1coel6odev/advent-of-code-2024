use std::collections::{HashMap, HashSet};

pub struct GuardPath {
    map: Vec<Vec<char>>,
    starting_pos: (usize, usize),
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Orientation {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl GuardPath {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let starting_pos = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                if let Some(x) = line.iter().enumerate().find_map(|(x, val)| {
                    if val.eq(&'^') {
                        return Some(x);
                    }
                    return None;
                }) {
                    return Some((x, y));
                }
                return None;
            })
            .unwrap();

        GuardPath::print_map(&map);
        println!("Starting pos: {:?}", starting_pos);

        GuardPath { map, starting_pos }
    }

    pub fn calculate_guard_path(&self) {
        let mut temp_map = self.map.clone();
        let temp_map = GuardPath::map_guard_path(&mut temp_map, self.starting_pos);
        GuardPath::print_map(&temp_map);
        println!(
            "Unique positions: {}",
            &temp_map
                .iter()
                .map(|line| line
                    .iter()
                    .map(|c| {
                        if c == &'X' {
                            return 1;
                        } else {
                            return 0;
                        }
                    })
                    .sum::<usize>())
                .sum::<usize>()
        )
    }

    pub fn calculate_possible_guard_loops(&self) {
        let mut temp_map = self.map.clone();
        let nr_loops = GuardPath::map_guard_path_with_loops(&mut temp_map, self.starting_pos);
        println!("Nr of loops: {}", nr_loops);
    }

    pub fn map_guard_path(
        map: &mut Vec<Vec<char>>,
        starting_pos: (usize, usize),
    ) -> Vec<Vec<char>> {
        let mut current_orientation = Orientation::UP;
        let mut current_position = starting_pos;
        map[current_position.1][current_position.0] = 'X';

        loop {
            let next_pos =
                GuardPath::get_next_position(&map, current_position, &current_orientation);

            if let Some((next_position, next_orientation)) = next_pos {
                map[next_position.1][next_position.0] = 'X';
                current_position = next_position;
                current_orientation = next_orientation;
            } else {
                break;
            }
        }

        map.to_vec()
    }

    pub fn map_guard_path_with_loops(
        map: &mut Vec<Vec<char>>,
        starting_pos: (usize, usize),
    ) -> usize {
        let mut current_orientation = Orientation::UP;
        let mut current_position = starting_pos.clone();
        let mut loop_position = HashSet::new();

        loop {
            let next_pos =
                GuardPath::get_next_position(&map, current_position, &current_orientation);

            if let Some((next_position, next_orientation)) = next_pos {
                current_position = next_position;
                current_orientation = next_orientation;

                let next_pos_obs =
                    GuardPath::get_next_position(&map, current_position, &current_orientation);

                if let Some((obs_pos, _)) = next_pos_obs {
                    if obs_pos != starting_pos {
                        let mut fake_map = map.clone();
                        fake_map[obs_pos.1][obs_pos.0] = '#';
                        let it_loops = GuardPath::check_creates_loop(
                            &fake_map,
                            starting_pos.clone(),
                            Orientation::UP,
                        );
                        if it_loops {
                            loop_position.insert(obs_pos);
                        }
                    }
                }
            } else {
                break;
            }
        }

        loop_position.len()
    }

    pub fn check_creates_loop(
        map: &Vec<Vec<char>>,
        starting_pos: (usize, usize),
        start_orientation: Orientation,
        // known_loop_locations: &mut HashMap<(usize, usize), HashSet<Orientation>>,
    ) -> bool {
        let mut current_orientation = start_orientation;
        let mut current_position = starting_pos;

        let mut possible_loop_positions: HashMap<(usize, usize), HashSet<Orientation>> =
            HashMap::new();
        possible_loop_positions.insert(
            current_position,
            HashSet::from([current_orientation.clone()]),
        );

        loop {
            let next_pos =
                GuardPath::get_next_position(&map, current_position, &current_orientation);

            if let Some((next_position, next_orientation)) = next_pos {
                // if let Some(orientations) = known_loop_locations.get(&next_position) {
                //     if orientations.contains(&next_orientation) {
                //         return true;
                //     }
                // }
                if let Some(orientations) = possible_loop_positions.get(&next_position) {
                    if orientations.contains(&next_orientation) {
                        return true;
                    }
                }
                current_orientation = next_orientation;
                current_position = next_position;
                possible_loop_positions
                    .entry(current_position)
                    .or_insert(HashSet::new())
                    .insert(current_orientation.clone());
            } else {
                return false;
            }
        }
    }

    pub fn get_next_position(
        map: &Vec<Vec<char>>,
        current_pos: (usize, usize),
        orientation: &Orientation,
    ) -> Option<((usize, usize), Orientation)> {
        let new_pos: (usize, usize);
        let fallback_orientation: Orientation;
        match orientation {
            Orientation::UP => {
                if current_pos.1 == 0 {
                    return None;
                }
                new_pos = (current_pos.0, current_pos.1 - 1);
                fallback_orientation = Orientation::RIGHT;
            }
            Orientation::RIGHT => {
                if current_pos.0 == map[0].len() - 1 {
                    return None;
                }
                new_pos = (current_pos.0 + 1, current_pos.1);
                fallback_orientation = Orientation::DOWN;
            }
            Orientation::DOWN => {
                if current_pos.1 == map.len() - 1 {
                    return None;
                }
                new_pos = (current_pos.0, current_pos.1 + 1);
                fallback_orientation = Orientation::LEFT;
            }
            Orientation::LEFT => {
                if current_pos.0 == 0 {
                    return None;
                }
                new_pos = (current_pos.0 - 1, current_pos.1);
                fallback_orientation = Orientation::UP;
            }
        }

        if map[new_pos.1][new_pos.0] == '#' {
            return GuardPath::get_next_position(&map, current_pos, &fallback_orientation);
        }
        return Some((new_pos, orientation.clone()));
    }

    pub fn print_map(map: &Vec<Vec<char>>) {
        println!("vvvvvvvvvv");
        for line in map {
            for character in line {
                print!("{}", character);
            }
            println!()
        }
    }
}
