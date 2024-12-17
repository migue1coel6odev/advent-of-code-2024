use aoc_utils::file_read::read_file_as_string;
use warehouse::Warehouse;
use warehouse_v2::Warehousev2;

mod warehouse;
mod warehouse_v2;

fn main() {
    // part_1("input.txt");
    part_2("input.txt");
}

#[allow(dead_code)]
fn part_2(path: &str) -> usize {
    let (map, instructions) = read_file(path);

    let mut warehouse = Warehousev2::new(&map);
    warehouse.run_instructions(instructions);
    let result = warehouse.calculate_boxes_gps_coordinates();
    println!("| PART 2 | {}", result);
    result
}

#[allow(dead_code)]
fn part_1(path: &str) -> usize {
    let (map, instructions) = read_file(path);

    let mut warehouse = Warehouse::new(&map);
    warehouse.run_instructions(instructions);
    let result = warehouse.calculate_boxes_gps_coordinates();
    println!("| PART 1 | {}", result);
    result
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

    // #[test]
    // fn test_part_1() {
    //     let result = part_1("test.txt");
    //     assert_eq!(result, 10092);
    // }

    #[test]
    fn test_part_2() {
        let result = part_2("test.txt");
        assert_eq!(result, 9021);
    }
}
