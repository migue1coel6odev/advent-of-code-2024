use std::{cmp::Ordering, ops::Sub};

pub struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    pub fn new(button_a: (usize, usize), button_b: (usize, usize), prize: (usize, usize)) -> Self {
        Self {
            button_a,
            button_b,
            prize: (prize.0 + 10000000000000, prize.1 + 10000000000000),
        }
    }

    pub fn calculate_best_token_usage(&self) -> usize {
        let result: Option<(usize, usize)> = match self.button_b.0.cmp(&self.button_b.1) {
            Ordering::Greater => {
                let mut start_with = self.prize.0 / self.button_b.0;
                let result: Option<(usize, usize)>;
                loop {
                    let possible_ratio = Machine::calculate_possible_ration(
                        &mut start_with,
                        self.button_b.0,
                        self.button_a.0,
                        self.prize.0,
                    );

                    if let Some((b_clicks, a_clicks)) = possible_ratio {
                        if a_clicks * self.button_a.1 + *b_clicks * self.button_b.1 == self.prize.1
                        {
                            result = Some((a_clicks, *b_clicks));
                            break;
                        }
                    } else {
                        result = None;
                        break;
                    }
                    if start_with == 0 {
                        result = None;
                        break;
                    }
                    start_with -= 1;
                }
                result
            }
            _ => {
                let mut start_with = self.prize.1 / self.button_b.1;
                let result: Option<(usize, usize)>;
                loop {
                    let possible_ratio = Machine::calculate_possible_ration(
                        &mut start_with,
                        self.button_b.1,
                        self.button_a.1,
                        self.prize.1,
                    );

                    if let Some((b_clicks, a_clicks)) = possible_ratio {
                        if a_clicks * self.button_a.0 + *b_clicks * self.button_b.0 == self.prize.0
                        {
                            result = Some((a_clicks, *b_clicks));
                            break;
                        }
                    } else {
                        result = None;
                        break;
                    }
                    if start_with == 0 {
                        result = None;
                        break;
                    }
                    start_with -= 1;
                }
                result
            }
        };

        if let Some((a, b)) = result {
            return a * 3 + b;
        }
        return 0;
    }

    pub fn calculate_possible_ration<'a>(
        start_with: &'a mut usize,
        a: usize,
        b: usize,
        prize: usize,
    ) -> Option<(&'a mut usize, usize)> {
        let rest = prize - (a * *start_with);

        if rest % b == 0 {
            return Some((start_with, rest / b));
        }

        if *start_with == 0 {
            return None;
        }

        *start_with -= 1;

        return Machine::calculate_possible_ration(start_with, a, b, prize);
    }
}
