use crate::{command::pause, coord::Coord, display::display_matrix};

pub struct CharMatrix {
    matrix: Vec<Vec<char>>,
}

impl CharMatrix {
    pub fn new(matrix: Vec<Vec<char>>) -> Self {
        Self { matrix }
    }

    pub fn find_reachable_area(
        &self,
        starting_coord: Coord,
        radius: usize,
        lookfor: Vec<char>,
        count: Vec<char>,
    ) -> (Vec<Coord>, usize) {
        let mut temp_matrix = self.matrix.clone();
        starting_coord.mark_coord_as_visited_with(&mut temp_matrix, '@');
        let mut queue: Vec<Vec<Coord>> = vec![vec![starting_coord]];
        let mut current_radius = 0;
        let mut current_count = 0;

        while current_radius < radius || queue.len() == 0 {
            let mut temp_queue = vec![];

            for coord in &queue[current_radius] {
                let surrondings = coord.check_surroundings(&temp_matrix, lookfor.clone());

                for (_, coord) in surrondings {
                    if count.contains(&coord.check_char_at(&temp_matrix)) {
                        current_count += 1;
                    }
                    coord.mark_coord_as_visited_with(&mut temp_matrix, '@');
                    temp_queue.push(coord);
                }
            }
            current_radius += 1;
            queue.push(temp_queue);
        }

        (
            queue
                .iter()
                .flat_map(|iter| iter.clone())
                .collect::<Vec<Coord>>(),
            current_count,
        )
    }
}
