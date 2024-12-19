use crate::{check::check_surroundings_char, maze::points::Point};

pub struct Maze<'a, 'b, 'c> {
    map: &'a Vec<Vec<char>>,
    start_pos: &'b (usize, usize),
    end_pos: &'c (usize, usize),
}

impl<'a, 'b, 'c> Maze<'a, 'b, 'c> {
    pub fn new(
        map: &'a Vec<Vec<char>>,
        start_pos: &'b (usize, usize),
        end_pos: &'c (usize, usize),
    ) -> Self {
        Self {
            map,
            start_pos,
            end_pos,
        }
    }

    pub fn find_fastest_path(&self) -> Option<Vec<Point>> {
        let mut temp_map = self.map.clone();

        let start_pos = Point {
            x: self.start_pos.0,
            y: self.start_pos.1,
        };
        let mut queue: Vec<(Point, Vec<Point>)> = vec![(start_pos, vec![])];

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

    fn check_next_point(
        &self,
        queue: &mut Vec<(Point, Vec<Point>)>,
        map: &mut Vec<Vec<char>>,
        point: Point,
        path: Vec<Point>,
    ) -> Option<Vec<Point>> {
        Maze::mark_visited(map, &point);

        if let Some(surrondings) =
            check_surroundings_char(&map, (point.x, point.y), vec!['.'], vec![])
        {
            for (_, pos, _) in surrondings {
                let mut path = path.clone();
                let point = Point { x: pos.0, y: pos.1 };
                path.push(point);
                Maze::mark_visited(map, &point);
                if &pos == self.end_pos {
                    return Some(path);
                }
                queue.push((Point { x: pos.0, y: pos.1 }, path));
            }
        }

        None
    }

    fn mark_visited(map: &mut Vec<Vec<char>>, point: &Point) {
        map[point.y][point.x] = 'X';
    }
}

#[cfg(test)]
mod test {
    use super::Maze;

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
        let maze = Maze::new(&parsed_input, &(0, 0), &(6, 6));
        assert_eq!(maze.find_fastest_path().unwrap().len(), 22);
    }
}
