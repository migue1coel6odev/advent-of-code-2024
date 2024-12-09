#[derive(Debug)]
pub struct Equation {
    result: usize,
    values: Vec<usize>,
}

impl Equation {
    pub fn new(result: usize, values: Vec<usize>) -> Self {
        Equation { result, values }
    }

    pub fn resolve(&self) -> usize {

        let val = *self.values.first().unwrap();

        let mut values_clone = self.values.clone();

        values_clone.remove(0);

        if Equation::_resolve(
            &self.result,
            &mut values_clone.clone(),
            val,
            format!("{}", val)
        ){
            println!("This -> {}",  self.result);
            return self.result;
        }

        return 0;
    }

    fn _resolve(result: &usize, values: &mut Vec<usize>, current_value: usize, text: String) -> bool {
        if values.len() == 0 && &current_value == result {
            println!("Result : {} with: {}", result, text);
            return true;
        }

        if values.len() == 0 || current_value > *result {
            return false;
        }

        let val = *values.first().unwrap();

        values.remove(0);

        return Equation::_resolve(result, &mut values.clone(), val + current_value, format!("{} + {}", text, val ))
            || Equation::_resolve(result, &mut values.clone(), val * current_value, format!("{} * {}", text, val));
    }
}
