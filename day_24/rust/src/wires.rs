use std::collections::HashMap;

pub struct Wires {
    wire_values: HashMap<String, usize>,
    wires_logic: Vec<String>
}

impl Wires {

    pub fn new(initial_wires_values: HashMap<String,usize>) -> Wires {
        Self {
            wire_values: initial_wires_values,
            wires_logic: Vec::new()
        }
    }

    pub fn decode_wires(&self) -> Vec<String> {
        unimplemented!()
    }

}