use crate::coord::Coord;

pub fn find_coord_of_char(matrix: &Vec<Vec<char>>, character: &char) -> Option<(usize, usize)> {
    matrix.iter().enumerate().find_map(|(y, line)| {
        line.iter()
            .enumerate()
            .find_map(|(x, c)| if c == character { Some((x, y)) } else { None })
    })
}

pub fn find_coord_of_char_as_coord(matrix: &Vec<Vec<char>>, character: &char) -> Option<Coord> {
    if let Some((x, y)) = find_coord_of_char(matrix, character) {
        return Some(Coord { x, y });
    }
    return None;
}
