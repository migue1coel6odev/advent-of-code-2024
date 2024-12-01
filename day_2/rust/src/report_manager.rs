use crate::report::Report;

pub struct ReportManager {
    report_list: Vec<Report>,
}

impl ReportManager {
    pub fn new(reports: Vec<String>) -> Self {
        ReportManager {
            report_list: reports.iter().map(|report| Report::new(report)).collect(),
        }
    }

    pub fn check_safe_reports(&self) -> usize {
        self.report_list.iter().filter(|report| report.check_is_safe()).collect::<Vec<&Report>>().len()
    }

    pub fn check_almost_safe_reports(&self) -> usize {
        self.report_list.iter().filter(|report| report.check_is_almost_safe()).collect::<Vec<&Report>>().len()
    }
}
