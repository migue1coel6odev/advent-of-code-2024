use aoc_utils::maze::maze::Maze;

pub struct Memory {
    corrupted_pos: Vec<(usize, usize)>,
    memory_map: Vec<Vec<char>>,
    end_pos: (usize, usize),
}

impl Memory {
    pub fn new(corruped_pos: &Vec<(usize, usize)>, max_distance: usize) -> Self {
        Self {
            corrupted_pos: corruped_pos.clone(),
            memory_map: vec![vec!['.'; max_distance + 1]; max_distance + 1],
            end_pos: (max_distance, max_distance),
        }
    }

    pub fn mark_corrupted_bytes(&mut self, nr_bytes: usize) {
        let mut byte = 0;
        while byte != nr_bytes {
            let (x, y) = self.corrupted_pos.remove(0);
            self.memory_map[y][x] = '#';
            byte += 1;
        }
    }

    pub fn find_fastest_path(&self) -> Option<usize> {
        let maze = Maze::new(&self.memory_map, &(0, 0), &self.end_pos);
        if let Some(result) = maze.find_fastest_path() {
            return Some(result.len());
        }
        None
    }
}
