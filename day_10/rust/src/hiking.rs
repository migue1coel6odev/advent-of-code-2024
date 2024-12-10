use std::{collections::HashSet, thread::sleep, time::Duration};

pub struct Hiking {
    map: Vec<Vec<usize>>,
    pub head: (usize, usize),
    queue: Vec<((usize, usize), usize)>,
    paths_found: HashSet<(usize, usize)>,
    pub rating: usize,
}

impl Hiking {
    pub fn new(map: Vec<Vec<usize>>, head: (usize, usize)) -> Self {
        Hiking {
            map,
            head,
            queue: Vec::new(),
            paths_found: HashSet::new(),
            rating: 0,
        }
    }

    pub fn discover(&mut self) -> usize {
        self.queue.push(((self.head), 1));

        let mut index = 0;

        loop {
            let queue_item = self.queue.get(index);

            if queue_item.is_none() {
                break;
            }

            let (pos, look_for) = queue_item.unwrap();
            self._discover(*pos, *look_for);

            index += 1;

            // self.print_queue();
        }

        self.paths_found.len()
    }

    pub fn print_queue(&self) {
        let mut temp_map = self
            .map
            .clone()
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&val| char::from_digit(val as u32, 10).unwrap())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        self.queue
            .iter()
            .for_each(|(pos, _)| temp_map[pos.1][pos.0] = '-');

        println!("----");
        for line in temp_map {
            for nr in line {
                print!("{}", nr);
            }
            println!();
        }
        println!("----");
        sleep(Duration::from_secs(1));
    }

    fn _discover(&mut self, current_pos: (usize, usize), look_for: usize) {
        let mut found = Vec::new();

        if current_pos.0 >= 1 && self.map[current_pos.1][current_pos.0 - 1] == look_for {
            found.push((current_pos.0 - 1, current_pos.1));
        }
        if current_pos.0 < self.map[0].len() - 1
            && self.map[current_pos.1][current_pos.0 + 1] == look_for
        {
            found.push((current_pos.0 + 1, current_pos.1));
        }
        if current_pos.1 >= 1 && self.map[current_pos.1 - 1][current_pos.0] == look_for {
            found.push((current_pos.0, current_pos.1 - 1));
        }
        if current_pos.1 < self.map.len() - 1
            && self.map[current_pos.1 + 1][current_pos.0] == look_for
        {
            found.push((current_pos.0, current_pos.1 + 1));
        }

        if look_for == 9 {
            found.iter().for_each(|&pos| {
                self.paths_found.insert(pos);
            });
            self.rating += found.len();
            return;
        }

        // if found.len() > 1 {
        //     self.rating += found.len();
        // }

        found
            .iter()
            .for_each(|&pos| self.queue.push((pos, look_for + 1)));
    }

    pub fn get_trailheads_as_hiking(input: Vec<Vec<usize>>) -> Vec<Hiking> {
        let mut trailheads = Vec::new();
        input.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, height)| {
                if height == &0 {
                    trailheads.push(Hiking::new(input.clone(), (x, y)));
                }
            })
        });
        return trailheads;
    }

    pub fn get_trailheads(input: Vec<Vec<usize>>) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        input.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, height)| {
                if height == &0 {
                    trailheads.push((x, y));
                }
            })
        });
        return trailheads;
    }
}
