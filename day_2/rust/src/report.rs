pub struct Report {
    report: Vec<isize>,
}

impl Report {
    pub fn new(report: &String) -> Self {
        let report: Vec<isize> = report
            .split(" ")
            .into_iter()
            .map(|value| value.parse::<isize>().unwrap())
            .collect();

        Report { report }
    }

    fn _check_is_safe(report_list: &Vec<isize>) -> bool {
        let is_increasing = report_list.first().unwrap() < report_list.last().unwrap();

        let range = 1..4;
        let mut is_safe = true;

        let _ = report_list
            .iter()
            .zip(report_list.iter().skip(1))
            .for_each(|(first, second)| {
                let mut diff = first - second;
                if is_increasing {
                    diff = second - first;
                }

                if !range.contains(&diff) {
                    is_safe = false;
                }
            });

        is_safe
    }

    pub fn check_is_safe(&self) -> bool {
        Report::_check_is_safe(&self.report)
    }

    pub fn check_is_almost_safe(&self) -> bool {
        let x = (0..self.report.len()).filter(|current_omit_index| Report::_check_is_safe(
            &self.report.iter().enumerate().filter(|(index,_)| *index != *current_omit_index).map(|(_, value)| *value).collect::<Vec<isize>>()
        )).collect::<Vec<usize>>();

        x.len() != 0
    }
}
