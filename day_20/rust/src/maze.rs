use aoc_utils::{
    check::check_surroundings_char, command::pause, coord::Coord, display::display_matrix,
};

pub struct Maze<'a, 'b> {
    map: &'a Vec<Vec<char>>,
    start_pos: &'b Coord,
    end_char: Vec<char>,
    marker: char,
    path: Vec<char>,
    restricted_coord: Vec<Coord>,
}

impl<'a, 'b> Maze<'a, 'b> {
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

    pub fn find_all_paths(
        &self,
        max_paths: usize,
        max_depth: usize,
        vec: Vec<Coord>,
    ) -> Vec<Vec<Coord>> {
        let mut temp_map = self.map.clone();
        let mut found_paths = vec![];
        let mut queue: Vec<(Coord, Vec<Coord>)> =
            vec![(self.start_pos.clone(), vec![self.start_pos.clone()])];
        self.mark_visited(&mut temp_map, self.start_pos);
        let mut vec = vec.clone();
        let mut current_path_length = 1;

        loop {
            if queue.len() == 0 {
                break;
            }
            if found_paths.len() == max_paths {
                break;
            }

            let (point, path) = queue.remove(0);
            if current_path_length != path.len() {
                if vec.len() > 0 {
                    if current_path_length > 0 {
                        self.mark_visited(&mut temp_map, &vec.remove(0));
                    }
                }
            }
            if temp_map
                .iter()
                .find(|line| line.iter().filter(|c| self.end_char.contains(c)).count() > 0)
                .is_none()
            {
                break;
            }
            current_path_length = path.len();
            if current_path_length > max_depth {
                break;
            }
            let val = self.check_next_point(&mut queue, &mut temp_map, point, path);
            if !val.is_empty() {
                val.iter().for_each(|v| {
                    found_paths.push(v.clone());
                });
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
    ) -> Vec<Vec<Coord>> {
        let mut values_found = vec![];

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
                        values_found.push(path);
                    } else {
                        queue.push((Coord { x: pos.0, y: pos.1 }, path));
                    }
                    self.mark_visited(map, &point);
                }
            }
        }

        values_found
    }

    fn mark_visited(&self, map: &mut Vec<Vec<char>>, point: &Coord) {
        map[point.y][point.x] = self.marker;
    }
}
