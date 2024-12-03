use std::fs;
use report_manager::ReportManager;

mod report_manager;
mod report;

fn main() {
    let reports = read_file();

    let report_manager = ReportManager::new(reports);

    println!("Number of safe reports is {}", report_manager.check_safe_reports());
    println!("Number of almost safe reports is {}", report_manager.check_almost_safe_reports());



}

fn read_file() -> Vec<String> {
    String::from_utf8(fs::read("input.txt").unwrap())
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    .collect()
}  