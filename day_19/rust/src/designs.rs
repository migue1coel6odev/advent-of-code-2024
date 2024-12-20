use std::collections::HashMap;

pub struct Design {
    available_patterns: Vec<String>,
    patterns_by_first_color: HashMap<char, Vec<String>>,
}

impl Design {
    pub fn new(available_patterns: Vec<String>) -> Self {
        Self {
            patterns_by_first_color: Design::distribute_patterns_by_color(&available_patterns),
            available_patterns,
        }
    }

    pub fn is_design_possible(&self, desired_design: &String) -> bool {
        if desired_design.is_empty() {
            return true;
        }

        let first_char = desired_design.chars().collect::<Vec<_>>()[0];
        if let Some(possible_patterns) = self.patterns_by_first_color.get(&first_char) {
            if possible_patterns
                .iter()
                .find(|f| {
                    if desired_design.starts_with(*f) {
                        return self
                            .is_design_possible(&desired_design.clone().replacen(*f, "", 1));
                    }

                    false
                })
                .is_some()
            {
                return true;
            };
        }

        false
    }

    pub fn find_all_ways_possible(&self, desired_design: &String) -> usize {
        let patterns_by_first_color = self
            .patterns_by_first_color
            .iter()
            .map(|(key, value)| {
                (
                    *key,
                    value
                        .iter()
                        .filter(|pat| desired_design.contains(*pat))
                        .map(|f| f.clone())
                        .collect::<Vec<String>>(),
                )
            })
            .collect::<HashMap<char, Vec<String>>>();

        println!(
            "diff from {} to {}",
            self.patterns_by_first_color.get(&'w').unwrap().len(),
            patterns_by_first_color.get(&'w').unwrap().len()
        );

        self.ways_possible(&patterns_by_first_color, desired_design)
    }

    fn ways_possible(
        &self,
        patterns_by_first_color: &HashMap<char, Vec<String>>,
        desired_design: &String,
    ) -> usize {
        if desired_design.is_empty() {
            return 1;
        }

        let first_char = desired_design.chars().collect::<Vec<_>>()[0];
        if let Some(possible_patterns) = patterns_by_first_color.get(&first_char) {
            return possible_patterns
                .iter()
                .map(|f| {
                    if desired_design.starts_with(f) {
                        return self.ways_possible(
                            &patterns_by_first_color,
                            &desired_design.clone().replacen(f, "", 1),
                        );
                    }

                    return 0;
                })
                .sum::<usize>();
        }
        return 0;
    }

    fn distribute_patterns_by_color(
        available_patterns: &Vec<String>,
    ) -> HashMap<char, Vec<String>> {
        let mut hashmap: HashMap<char, Vec<String>> = HashMap::new();

        available_patterns.iter().for_each(|f| {
            let first_char = f.chars().collect::<Vec<char>>()[0];
            hashmap
                .entry(first_char)
                .and_modify(|vec| vec.push(f.clone()))
                .or_insert(vec![f.clone()]);
        });

        hashmap
    }
}

#[cfg(test)]
mod test {
    use super::Design;

    #[test]
    fn test_distribute_patters_by_color() {
        let ap = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let result = Design::distribute_patterns_by_color(
            &ap.iter().map(|f| f.to_string()).collect::<Vec<String>>(),
        );
        assert_eq!(
            result.get(&'w'),
            Some(
                &vec!["wr"]
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
            )
        );
    }
}
