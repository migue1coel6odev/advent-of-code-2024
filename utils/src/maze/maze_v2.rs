use crate::{
    check::check_surroundings_char, command::pause, coord::Coord, display::display_matrix,
};

pub struct Mazev2<'a, 'b, 'c> {
    map: &'a Vec<Vec<char>>,
    start_pos: &'b Coord,
    end_pos: &'c Coord,
}

impl<'a, 'b, 'c> Mazev2<'a, 'b, 'c> {
    pub fn new(map: &'a Vec<Vec<char>>, start_pos: &'b Coord, end_pos: &'c Coord) -> Self {
        Self {
            map,
            start_pos,
            end_pos,
        }
    }

    pub fn find_fastest_path(&self) -> Option<Vec<Coord>> {
        let mut temp_map = self.map.clone();

        let mut queue: Vec<(Coord, Vec<Coord>)> =
            vec![(self.start_pos.clone(), vec![self.start_pos.clone()])];

        loop {
            if queue.len() == 0 {
                return None;
            }

            let (point, path) = queue.remove(0);
            if let Some(val) = self.check_next_point(&mut queue, &mut temp_map, point, path) {
                return Some(val);
            }
        }
    }

    pub fn find_all_paths(&self) -> Vec<Vec<Coord>> {
        let mut temp_map = self.map.clone();
        let mut found_paths = vec![];
        let mut queue: Vec<(Coord, Vec<Coord>)> =
            vec![(self.start_pos.clone(), vec![self.start_pos.clone()])];

        loop {
            if queue.len() == 0 {
                break;
            }

            let (point, path) = queue.remove(0);
            if let Some(val) = self.check_next_point(&mut queue, &mut temp_map, point, path) {
                found_paths.push(val);
            }
        }
        found_paths
    }

    fn check_next_point(
        &self,
        queue: &mut Vec<(Coord, Vec<Coord>)>,
        map: &mut Vec<Vec<char>>,
        point: Coord,
        path: Vec<Coord>,
    ) -> Option<Vec<Coord>> {
        Mazev2::mark_visited(map, &point);

        if let Some(surrondings) =
            check_surroundings_char(&map, (point.x, point.y), vec!['.', 'E'], vec![])
        {
            for (_, pos, _) in surrondings {
                let mut path = path.clone();
                let point = Coord { x: pos.0, y: pos.1 };
                path.push(point);
                Mazev2::mark_visited(map, &point);
                if &point == self.end_pos {
                    return Some(path);
                }
                queue.push((Coord { x: pos.0, y: pos.1 }, path));
            }
        }

        None
    }

    fn mark_visited(map: &mut Vec<Vec<char>>, point: &Coord) {
        map[point.y][point.x] = 'X';
    }
}

#[cfg(test)]
mod test {
    use crate::coord::Coord;

    use super::Mazev2;

    #[test]
    fn test_maze() {
        let input = r"...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....";
        let parsed_input = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let maze = Mazev2::new(&parsed_input, &Coord { x: 0, y: 0 }, &Coord { x: 6, y: 6 });
        assert_eq!(maze.find_fastest_path().unwrap().len(), 22);
    }

    #[test]
    fn test_maze_S_E() {
        let input = r"...#...
S.#..#.
....#..
...#..#
..#..#.
.#..#..
#.#...E";
        let parsed_input = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let maze = Mazev2::new(&parsed_input, &Coord { x: 0, y: 0 }, &Coord { x: 6, y: 6 });
        assert_eq!(maze.find_fastest_path().unwrap().len(), 22);
    }
}
