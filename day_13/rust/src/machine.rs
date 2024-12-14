
pub struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize)
}

impl Machine {

    pub fn new(button_a: (usize, usize), button_b: (usize, usize), prize: (usize, usize)) -> Self {
        Self {
            button_a,
            button_b,
            prize
        }
    }


    pub fn calculate_best_token_usage(&self) -> usize {

        unimplemented!()
    }


}