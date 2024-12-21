use std::ops::Sub;

use crate::check::MatrixPos;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn exists_in_matrix(&self, matrix: &Vec<Vec<char>>) -> bool {
        return self.x > 0
            && self.y > 0
            && self.x < matrix[0].len() - 1
            && self.y < matrix.len() - 1;
    }

    pub fn check_char_at(&self, matrix: &Vec<Vec<char>>) -> char {
        matrix[self.y][self.x]
    }

    pub fn mark_coord_as_visited(&self, matrix: &mut Vec<Vec<char>>) {
        matrix[self.y][self.x] = 'X';
    }

    pub fn mark_coord_as_visited_with(&self, matrix: &mut Vec<Vec<char>>, marker: char) {
        matrix[self.y][self.x] = marker;
    }

    pub fn move_in_dir(&self, direction: MatrixPos) -> Option<Self> {
        let mut x = None;
        let mut y = None;
        match direction {
            MatrixPos::UP => {
                x = Some(self.x);
                y = self.y.checked_sub(1);
            }
            MatrixPos::DOWN => {
                x = Some(self.x);
                y = self.y.checked_add(1);
            }
            MatrixPos::LEFT => {
                x = self.x.checked_sub(1);
                y = Some(self.y);
            }
            MatrixPos::RIGHT => {
                x = self.x.checked_add(1);
                y = Some(self.y);
            }
        };
        if x.is_some() && y.is_some() {
            return Some(Coord {
                x: x.unwrap(),
                y: y.unwrap(),
            });
        }
        return None;
    }

    pub fn check_next_coord_by_dir(
        &self,
        matrix: &Vec<Vec<char>>,
        direction: MatrixPos,
    ) -> Option<(Coord, char)> {
        if let Some(new_coord) = self.move_in_dir(direction) {
            if new_coord.exists_in_matrix(matrix) {
                return Some((new_coord, new_coord.check_char_at(matrix)));
            }
        }
        return None;
    }

    pub fn check_surroundings(
        &self,
        matrix: &Vec<Vec<char>>,
        look_for: Vec<char>,
    ) -> Vec<(MatrixPos, Coord)> {
        vec![
            (
                MatrixPos::UP,
                self.check_next_coord_by_dir(matrix, MatrixPos::UP),
            ),
            (
                MatrixPos::DOWN,
                self.check_next_coord_by_dir(matrix, MatrixPos::DOWN),
            ),
            (
                MatrixPos::LEFT,
                self.check_next_coord_by_dir(matrix, MatrixPos::LEFT),
            ),
            (
                MatrixPos::RIGHT,
                self.check_next_coord_by_dir(matrix, MatrixPos::RIGHT),
            ),
        ]
        .iter()
        .filter(|(_, next_coord)| {
            if let Some(next_coord) = next_coord {
                if look_for.contains(&next_coord.1) {
                    return true;
                }
            }
            return false;
        })
        .map(|(dir, next_coord)| (dir.clone(), next_coord.unwrap().0))
        .collect::<Vec<(MatrixPos, Coord)>>()
    }

    pub fn check_surroundings_v2(&self, matrix: &Vec<Vec<char>>) -> Vec<Coord> {
        vec![
            self.check_next_coord_by_dir(matrix, MatrixPos::UP),
            self.check_next_coord_by_dir(matrix, MatrixPos::DOWN),
            self.check_next_coord_by_dir(matrix, MatrixPos::LEFT),
            self.check_next_coord_by_dir(matrix, MatrixPos::RIGHT),
        ]
        .iter()
        .filter(|res| res.is_some())
        .map(|next_coord| next_coord.unwrap().0)
        .collect::<Vec<Coord>>()
    }
}
