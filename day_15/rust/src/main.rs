use aoc_utils::file_read::read_file_as_string;
use warehouse::Warehouse;

mod warehouse;

fn main() {
    part_1("input.txt");
}

#[allow(dead_code)]
fn part_1(path: &str) {
    let (map, instructions) = read_file(path);

    let warehouse = Warehouse::new(&map);
}

fn read_file(path: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let input = read_file_as_string(path);
    let vec = input
        .split("\n\n")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    (
        vec[0]
            .split("\n")
            .map(|f| f.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
        vec[1].chars().collect::<Vec<char>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        part_1("test.txt");
    }
}
