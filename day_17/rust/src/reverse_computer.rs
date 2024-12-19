use crate::computer::Computer;

pub struct ReverseComputer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    pointer: usize,
    result: Vec<usize>,
}

impl ReverseComputer {
    pub fn new(
        register_a: usize,
        register_b: usize,
        register_c: usize,
        program: Vec<usize>,
        pointer: usize,
        result: Vec<usize>,
    ) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            program,
            pointer,
            result,
        }
    }

    pub fn run(&mut self) -> usize {}
}
