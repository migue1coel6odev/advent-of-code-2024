use std::{fs, thread::sleep, time};

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let mut input_matrix = read_file();
    let mut current_guard_pos: (usize, usize) = (0, 0);
    let mut count = 0;
    let mut count_1 = 0;
    let mut current_rotation_count = 0;
    let _ = input_matrix.iter().enumerate().find(|(index, line)| {
        line.iter()
            .enumerate()
            .find(|(x_index, c)| {
                if **c == '^' {
                    current_guard_pos = (*x_index, *index);
                    return true;
                }
                false
            })
            .is_some()
    });

    loop {
        let (x, y) = current_guard_pos;

        if input_matrix[y][x] == '.' {
            count_1 += 1;
        }

        input_matrix[y][x] = if input_matrix[y][x] == '>' || input_matrix[y][x] == '<' {
            '*'
        } else {
            '^'
        };

        if y == 0 {
            break;
        }

        current_guard_pos = (x, y - 1);
        let (x, y) = current_guard_pos;
        let mut found: Option<(usize, usize)> = None;

        if input_matrix[y][x] == '#' {
            input_matrix[y + 1][x] = '+';
            (input_matrix, current_guard_pos) = rotate_90_deg_2(&input_matrix, current_guard_pos);
            current_rotation_count += 1;
        } else {
            let (temp_matrix, new_current_guard_pos) = rotate_90_deg_2(&input_matrix, current_guard_pos);
            if check_possible_loop(&temp_matrix, new_current_guard_pos, 0) {
                found = Some((x, y));

                count += 1;
            }
        }
        if current_rotation_count == 4 {
            current_rotation_count = 0;
        }
        print_matrix(&input_matrix, current_rotation_count, None);
        if let Some((x, y)) = found {
            println!("Found one ");
            print_matrix(&input_matrix, current_rotation_count, Some((x, y)));
        }
        sleep(time::Duration::from_secs(1));
    }

    print_matrix(&input_matrix, current_rotation_count, None);

    println!("Sum = {}, 1 : {}", count, count_1);
}

fn check_possible_loop(matrix: &Vec<Vec<char>>, current_pos: (usize, usize), level: usize) -> bool {
    if level == 2 {
        return false;
    }
    for n in (0..current_pos.1 + 1).rev() {
        // print_matrix(&matrix);
        // println!("checking pos {:?}: {}", current_pos, matrix.get(n).unwrap().get(current_pos.0).unwrap() );
        match matrix.get(n).unwrap().get(current_pos.0).unwrap() {
            '#' => {
                let (new_matrix, current_pos) = rotate_90_deg_2(matrix, current_pos);
                return check_possible_loop(&new_matrix, current_pos, level + 1)
            }
            '^' | '+' => return true,
            _ => (),
        }
    }

    return false;
}

fn rotate_90_deg_2(matrix: &Vec<Vec<char>>, (x, y): (usize, usize)) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut new_matrix = Vec::new();
    for old_x in (0..matrix.get(0).unwrap().len()).rev() {
        let mut temp_vec = Vec::new();
        for old_y in 0..matrix.len() {
            temp_vec.push(match *matrix.get(old_y).unwrap().get(old_x).unwrap() {
                '^' => '<',
                'v' => '>',
                '>' => '^',
                '<' => 'v',
                c => c,
            });
        }
        new_matrix.push(temp_vec);
    }
    let new_matrix_len = new_matrix[0].len();
    println!("{} : {}", y , x);
    (new_matrix, (y + 1, (new_matrix_len - x) - 2))
}

fn rotate_180_deg(matrix: &Vec<Vec<char>>, current_pos: (usize, usize)) -> (Vec<Vec<char>>, (usize, usize)) {
    let (new_matrix, current_pos) = rotate_90_deg_2(matrix, current_pos);
    rotate_90_deg_2(&new_matrix, current_pos)
}

fn rotate_270_deg(matrix: &Vec<Vec<char>>, current_pos: (usize, usize)) -> (Vec<Vec<char>>, (usize, usize)) {
    let (new_matrix, current_pos) = rotate_90_deg_2(matrix, current_pos);
    let (new_matrix, current_pos) = rotate_90_deg_2(&new_matrix, current_pos);
    rotate_90_deg_2(&new_matrix, current_pos)
}

fn rotate_90_deg(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = Vec::new();
    for old_x in (0..matrix.get(0).unwrap().len()).rev() {
        let mut temp_vec = Vec::new();
        for old_y in 0..matrix.len() {
            temp_vec.push(*matrix.get(old_y).unwrap().get(old_x).unwrap());
        }
        new_matrix.push(temp_vec);
    }
    new_matrix
}

fn read_file() -> Vec<Vec<char>> {
    String::from_utf8(fs::read("test.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn print_matrix(
    matrix: &Vec<Vec<char>>,
    current_rotation_count: usize,
    highlight_pos: Option<(usize, usize)>,
) {
    println!("crc: {}", current_rotation_count);
    println!("\n");
    let mut c: Vec<Vec<char>> = matrix.clone();

    if let Some((x, y)) = highlight_pos {
        c[y][x] = 'O';
    }

    let m = match current_rotation_count {
        1 => rotate_270_deg(&c, highlight_pos.or(Some((0,0))).unwrap()),
        2 => rotate_180_deg(&c, highlight_pos.or(Some((0,0))).unwrap()),
        3 => rotate_90_deg_2(&c, highlight_pos.or(Some((0,0))).unwrap()),
        _ => (matrix.clone(), highlight_pos.or(Some((0,0))).unwrap()),
    };
    for n in m.0 {
        for ch in n {
            print!("{}", ch);
        }
        println!("");
    }
    println!("\n");
}

fn part_1() {
    let mut input_matrix = read_file();
    let mut current_guard_pos: (usize, usize) = (0, 0);
    let mut count = 0;
    let _ = input_matrix.iter().enumerate().find(|(index, line)| {
        line.iter()
            .enumerate()
            .find(|(x_index, c)| {
                if **c == '^' {
                    current_guard_pos = (*x_index, *index);
                    return true;
                }
                false
            })
            .is_some()
    });

    loop {
        if input_matrix[current_guard_pos.1][current_guard_pos.0] != 'X' {
            count += 1;
            input_matrix[current_guard_pos.1][current_guard_pos.0] = 'X';
        }

        if current_guard_pos.1 == 0 {
            break;
        }

        current_guard_pos = (current_guard_pos.0, current_guard_pos.1 - 1);

        if input_matrix[current_guard_pos.1][current_guard_pos.0] == '#' {
            input_matrix = rotate_90_deg(&input_matrix);
            current_guard_pos = (
                current_guard_pos.1 + 1,
                (input_matrix[0].len() - current_guard_pos.0) - 2,
            );
        }
    }

    println!("Sum = {}", count);
}
