#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MatrixPos {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn check_surroundings_char(
    matrix: &Vec<Vec<char>>,
    current_pos: (usize, usize),
    look_for: Vec<char>,
    ignore_pos: Vec<MatrixPos>,
) -> Option<Vec<(MatrixPos, (usize, usize), char)>> {
    let mut vec = Vec::new();

    if !ignore_pos.contains(&MatrixPos::UP)
        && current_pos.1 > 0
        && look_for.contains(&matrix[current_pos.1 - 1][current_pos.0])
    {
        vec.push((
            MatrixPos::UP,
            (current_pos.0, current_pos.1 - 1),
            matrix[current_pos.1 - 1][current_pos.0],
        ));
    }

    if !ignore_pos.contains(&MatrixPos::RIGHT)
        && current_pos.0 < matrix[0].len() - 1
        && look_for.contains(&matrix[current_pos.1][current_pos.0 + 1])
    {
        vec.push((
            MatrixPos::RIGHT,
            (current_pos.0 + 1, current_pos.1),
            matrix[current_pos.1][current_pos.0 + 1],
        ));
    }

    if !ignore_pos.contains(&MatrixPos::DOWN)
        && current_pos.1 < matrix.len() - 1
        && look_for.contains(&matrix[current_pos.1 + 1][current_pos.0])
    {
        vec.push((
            MatrixPos::DOWN,
            (current_pos.0, current_pos.1 + 1),
            matrix[current_pos.1 + 1][current_pos.0],
        ));
    }

    if !ignore_pos.contains(&MatrixPos::LEFT)
        && current_pos.0 > 0
        && look_for.contains(&matrix[current_pos.1][current_pos.0 - 1])
    {
        vec.push((
            MatrixPos::LEFT,
            (current_pos.0 - 1, current_pos.1),
            matrix[current_pos.1][current_pos.0 - 1],
        ));
    }

    if vec.len() > 0 {
        return Some(vec);
    }
    None
}

/**
 * @return [LEFT, RIGHT, TOP, BOTTOM]
 */
pub fn check_surroundings_char_per_pos(
    matrix: &Vec<Vec<char>>,
    current_pos: (usize, usize),
    look_for: Vec<char>,
) -> (usize, [Option<((usize, usize), char)>; 4]) {
    let mut arr = [None; 4];
    let mut count = 0;

    if current_pos.0 > 0 && look_for.contains(&matrix[current_pos.1][current_pos.0 - 1]) {
        arr[0] = Some((
            (current_pos.0 - 1, current_pos.1),
            matrix[current_pos.1][current_pos.0 - 1],
        ));
        count += 1;
    }

    if current_pos.0 < matrix[0].len() - 1
        && look_for.contains(&matrix[current_pos.1][current_pos.0 + 1])
    {
        arr[1] = Some((
            (current_pos.0 + 1, current_pos.1),
            matrix[current_pos.1][current_pos.0 + 1],
        ));
        count += 1;
    }

    if current_pos.1 > 0 && look_for.contains(&matrix[current_pos.1 - 1][current_pos.0]) {
        arr[2] = Some((
            (current_pos.0, current_pos.1 - 1),
            matrix[current_pos.1 - 1][current_pos.0],
        ));
        count += 1;
    }

    if current_pos.1 < matrix.len() - 1
        && look_for.contains(&matrix[current_pos.1 + 1][current_pos.0])
    {
        arr[3] = Some((
            (current_pos.0, current_pos.1 + 1),
            matrix[current_pos.1 + 1][current_pos.0],
        ));
        count += 1;
    }

    (count, arr)
}

pub fn check_distance_between_points(point_a: &(usize, usize), point_b: &(usize, usize)) -> usize {
    point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
}

pub fn check_euclidian_distance_between_points(
    point_a: &(usize, usize),
    point_b: &(usize, usize),
) -> f64 {
    let x_a: f64 = convert(point_a.0).unwrap();
    let y_a: f64 = convert(point_a.1).unwrap();
    let x_b: f64 = convert(point_b.0).unwrap();
    let y_b: f64 = convert(point_b.1).unwrap();

    f64::sqrt((x_b - x_a).powf(2_f64) + (y_b - y_a).powf(2.0))
}

fn convert(x: usize) -> Result<f64, &'static str> {
    let result = x as f64;
    if result as usize != x {
        return Err("cannot convert");
    }
    Ok(result)
}
