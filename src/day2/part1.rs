use super::parse_input;

pub fn safe_report(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|pair| pair[0] > pair[1] && (pair[1] - pair[0]).abs() <= 3)
        || report
            .windows(2)
            .all(|pair| pair[0] < pair[1] && (pair[1] - pair[0]).abs() <= 3)
}

pub fn solve(input: &str) -> usize {
    let reports = parse_input(input);
    let mut safe: usize = 0;
    for report in reports {
        if safe_report(&report) {
            safe += 1;
        }
    }
    safe
}
