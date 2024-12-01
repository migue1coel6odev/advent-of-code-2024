use std::fs;

use rules::{Rules, UpgradeReturn};

mod rules;

fn main() {
    let input = read_file_rules();
    let updates = read_file_updates();

    let rules = Rules::new(input);

    let result = updates
        .iter()
        .map(|update_line| {
            let mut upgrade = update_line
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            match rules.check_upgrade(&upgrade) {
                UpgradeReturn::Success(val) => (val, 0),
                UpgradeReturn::Error(pos_to_switch) => {
                    (0, rules.fix_upgrade(&mut upgrade, pos_to_switch))
                }
            }
        })
        .collect::<Vec<(usize, usize)>>();

    println!(
        "| PART 1 | Sum equals to: {}",
        result.iter().map(|(val, _)| val).sum::<usize>()
    );
    println!(
        "| PART 2 | Sum equals to: {}",
        result.iter().map(|(_, val)| val).sum::<usize>()
    );
}

fn read_file_rules() -> Vec<String> {
    String::from_utf8(fs::read("input.rules.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

fn read_file_updates() -> Vec<String> {
    String::from_utf8(fs::read("input.updates.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
