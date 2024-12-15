use super::parse_input;
use super::part1::safe_report;

pub fn solve(input: &str) -> usize {
    let reports = parse_input(input);
    let mut safe: usize = 0;
    for report in reports {
        if safe_report(&report) {
            safe += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if safe_report(&new_report) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}
