use crate::{
    check::check_surroundings_char, command::pause, coord::Coord, display::display_matrix,
};

pub struct Mazev3<'a, 'b> {
    map: &'a Vec<Vec<char>>,
    start_pos: &'b Coord,
    end_char: Vec<char>,
    marker: char,
    path: Vec<char>,
    restricted_coord: Vec<Coord>,
}

impl<'a, 'b> Mazev3<'a, 'b> {
    pub fn new(
        map: &'a Vec<Vec<char>>,
        start_pos: &'b Coord,
        end_char: Vec<char>,
        path: Vec<char>,
        marker: char,
        restricted_coord: Vec<Coord>,
    ) -> Self {
        Self {
            map,
            start_pos,
            end_char,
            path,
            marker,
            restricted_coord,
        }
    }

    pub fn find_all_paths(&self, max_paths: usize) -> Vec<Vec<Coord>> {
        let mut temp_map = self.map.clone();
        let mut found_paths = vec![];
        let mut queue: Vec<(Coord, Vec<Coord>)> =
            vec![(self.start_pos.clone(), vec![self.start_pos.clone()])];

        loop {
            if queue.len() == 0 || found_paths.len() == max_paths {
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
        self.mark_visited(map, &point);
        let mut values_found = None;

        let mut lookfor = self.path.clone();
        let mut end_chars = self.end_char.clone();
        lookfor.append(&mut end_chars);
        if let Some(surrondings) =
            check_surroundings_char(&map, (point.x, point.y), lookfor, vec![])
        {
            for (_, pos, _) in surrondings {
                let point = Coord { x: pos.0, y: pos.1 };
                if self.restricted_coord.contains(&point) {
                    let mut path = path.clone();
                    path.push(point);

                    if self.end_char.contains(&point.check_char_at(&map)) {
                        values_found = Some(path);
                    } else {
                        queue.push((Coord { x: pos.0, y: pos.1 }, path));
                    }
                }
            }
        }

        values_found
    }

    fn mark_visited(&self, map: &mut Vec<Vec<char>>, point: &Coord) {
        map[point.y][point.x] = self.marker;
    }
}

// #[cfg(test)]
// mod test {
//     use crate::coord::Coord;

//     use super::Mazev3;

//     #[test]
//     fn test_maze_S_E() {
//         let input = r"S.
// ..";
//         let parsed_input = input
//             .split("\n")
//             .map(|line| line.chars().collect::<Vec<char>>())
//             .collect::<Vec<Vec<char>>>();
//         let maze = Mazev3::new(&parsed_input, &Coord { x: 0, y: 0 }, '.', vec!['.'], 'X');
//         assert_eq!(maze.find_all_paths().len(), 2);
//     }
// }
