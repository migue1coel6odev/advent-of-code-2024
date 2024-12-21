use aoc_utils::file_read::read_file_as_char_matrix;
use race::Race;
use race_v2::Racev2;

mod race;
mod race_v2;

fn main() {
    let x: usize = 10;
    let xx = x.sub(1);
    println!("xx {}", xx);
    // println!("| PART 1 | Result = {}", part_1("input.txt", 100));
    // println!("| PART 2 | Result = {}", part_2("input.txt", 100, 20));
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

#[allow(dead_code)]
fn part_2(path: &str, save_time: usize, max_depth: usize) -> usize {
    let input = read_file_as_char_matrix(path);

    let mut race = Racev2::new(input);
    race.set_max_cheat_depth(max_depth);
    race.set_save_at_least(save_time);
    let res = race.check_paths_that_saves();
    // race.display_possible_cheats();
    res
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("test.txt", 12), 8)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("test.txt", 12, 2), 8);
        assert_eq!(part_2("test.txt", 50, 6), 285);
    }
}
