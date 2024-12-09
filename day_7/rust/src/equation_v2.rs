#[derive(Debug)]
pub struct Equationv2 {
    result: usize,
    values: Vec<usize>,
}

impl Equationv2 {
    pub fn new(result: usize, values: Vec<usize>) -> Self {
        Equationv2 { result, values }
    }

    pub fn resolve(&self) -> usize {

        let val = *self.values.first().unwrap();

        let mut values_clone = self.values.clone();

        values_clone.remove(0);

        if Equationv2::_resolve(
            &self.result,
            &mut values_clone.clone(),
            val,
            format!("{}", val)
        ) {
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

        return Equationv2::_resolve(result, &mut values.clone(), val + current_value, format!("{} + {}", text, val ))
            || Equationv2::_resolve(result, &mut values.clone(), val * current_value, format!("{} * {}", text, val))
            || Equationv2::_resolve(result, &mut values.clone(), format!("{}{}",current_value, val).parse::<usize>().unwrap(), format!("{} || {}", text, val));
    }
}
