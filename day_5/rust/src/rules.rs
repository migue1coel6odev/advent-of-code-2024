use std::collections::HashMap;

pub enum UpgradeReturn {
    Success(usize),
    Error((usize, usize)),
}

#[derive(Debug)]
pub struct Rules {
    rules: HashMap<usize, Vec<usize>>,
}

impl Rules {
    pub fn new(rule_list: Vec<String>) -> Self {
        let mut rules = HashMap::new();

        for line in rule_list {
            let mut rule = line.split('|');
            let left_value: usize = rule.next().unwrap().parse().unwrap();
            let right_value: usize = rule.next().unwrap().parse().unwrap();

            rules
                .entry(left_value)
                .or_insert_with(Vec::new)
                .push(right_value);
        }

        Rules { rules }
    }

    pub fn check_upgrade(&self, upgrade: &Vec<usize>) -> UpgradeReturn {
        let mut second_index = 0;
        let result = upgrade.iter().enumerate().skip(1).find(|(index, value)| {
            match self.rules.get(&value) {
                Some(vec) => {
                    for n in 0..*index + 1 {
                        if vec.contains(&upgrade[n]) {
                            second_index = n;
                            return true;
                        }
                    }

                    return false;
                }
                None => return false,
            }
        });
        if result.is_none() {
            return UpgradeReturn::Success(*upgrade.get(upgrade.len() / 2).unwrap());
        }

        UpgradeReturn::Error((result.unwrap().0, second_index))
    }

    pub fn fix_upgrade(&self, upgrade: &mut Vec<usize>, pos_to_switch: (usize, usize)) -> usize {
        let temp = *upgrade.get(pos_to_switch.0).unwrap();
        upgrade[pos_to_switch.0] = *upgrade.get(pos_to_switch.1).unwrap();
        upgrade[pos_to_switch.1] = temp;

        match self.check_upgrade(&upgrade) {
            UpgradeReturn::Error(pos_to_switch) => self.fix_upgrade(upgrade, pos_to_switch),
            UpgradeReturn::Success(val) => val,
        }
    }
}
