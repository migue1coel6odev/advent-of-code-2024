use regex::{Captures, Regex};

pub struct MagicStonesSuperFast {
    stones: String,
}

impl MagicStonesSuperFast {
    pub fn new(stones: String) -> Self {
        Self { stones }
    }

    pub fn blink(&mut self, blink_nr: usize) -> usize {
        let even_digits = Regex::new(r"\b(\d\d)+\b").unwrap();
        let zero = Regex::new(r"(\b0\b)").unwrap();
        let non_zero_odd_digits = Regex::new(r"(\b([1-9])\b)|(\b((\d\d)+\d)\b)").unwrap();
        let all = Regex::new(r"(\b(\d\d)+\b)|(\b0\b)|((\b([1-9])\b)|(\b((\d\d)+\d)\b))").unwrap();

        let replacement_all = |caps: &Captures| -> String {
            let val = &caps[0];
            if val == "0" {
                return String::from("1");
            }
            if val.chars().count() % 2 == 0 {
                let mut temp = vec![];
                let half = caps[0].chars().count() / 2;
                let nr = caps[0].parse::<usize>().unwrap();
                let pow = (10 as usize).pow(half as u32);
                temp.push(nr / pow);
                temp.push(nr % pow);

                return temp
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
            }

            return (caps[0].parse::<usize>().unwrap() * 2024).to_string();
        };

        let replacement_even = |caps: &Captures| -> String {
            let mut temp = vec![];
            let half = caps[0].chars().count() / 2;
            let nr = caps[0].parse::<usize>().unwrap();
            let pow = (10 as usize).pow(half as u32);
            temp.push(nr / pow);
            temp.push(nr % pow);

            temp.iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        };

        let replacement_nzod =
            |caps: &Captures| -> String { (caps[0].parse::<usize>().unwrap() * 2024).to_string() };

        let mut current_blink = 0;
        // println!("{}", self.stones);
        loop {
            if current_blink == blink_nr {
                break;
            }

            self.stones =
                String::from(&all.replace_all(&self.stones, &replacement_all).to_string());

            // println!(
            //     "cur: {} : {} ",
            //     current_blink,
            //     self.stones.split(" ").count()
            // );

            println!("{}", self.stones);

            // self.stones = String::from(&zero.replace_all(&self.stones, "1").to_string());

            // self.stones = String::from(
            //     &non_zero_odd_digits
            //         .replace_all(&self.stones, &replacement_nzod)
            //         .to_string(),
            // );

            // self.stones = String::from(
            //     &even_digits
            //         .replace_all(&self.stones, &replacement_even)
            //         .to_string(),
            // );

            current_blink += 1;
        }

        self.stones.split(" ").count()
    }
}
