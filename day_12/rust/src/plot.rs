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
        let area = self
            .map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|c| if c != &'.' { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>();

        unimplemented!();
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

        DisplayMap::display(&vec);

        vec
    }
}
