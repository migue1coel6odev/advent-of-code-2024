use aoc_utils::file_read::read_file_as_char_matrix;
use race::Race;

mod race;
mod race_v2;

fn main() {
    println!("| PART 1 | Result = {}", part_1("input.txt", 100));
}

#[allow(dead_code)]
fn part_1(path: &str, save_time: usize) -> usize {
    let input = read_file_as_char_matrix(path);

    let mut race = Race::new(input);
    let _ = race.check_paths_that_saves();
    let res = race.check_time_saved_per_cheat();

    res.iter()
        .filter(|time_saved| *time_saved >= &save_time)
        .count()
}

#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("test.txt", 12), 8)
    }
}
