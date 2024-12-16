use super::parse_grid;

pub fn solve(input: &str) -> usize {
    let mut total = 0;
    let grid = parse_grid(input);
    for row in &grid {
        total += row.matches("XMAS").count();
        total += row.matches("SAMX").count();
    }
    for i in 0..grid[0].len() {
        let mut col = String::new();
        for row in &grid {
            col.push(row.chars().nth(i).unwrap());
        }
        total += col.matches("XMAS").count();
        total += col.matches("SAMX").count();
    }
    // get diagonals
    let mut diag_1 = String::new();
    let mut diag_2 = String::new();
    for i in 0..grid.len() {
        diag_1.push(grid[i].chars().nth(i).unwrap());
        diag_2.push(grid[i].chars().nth(grid.len() - i - 1).unwrap());
    }
    total += diag_1.matches("XMAS").count();
    total += diag_1.matches("SAMX").count();
    total += diag_2.matches("XMAS").count();
    total += diag_2.matches("SAMX").count();
    for offset in 1..grid.len() {
        let mut diag_1 = String::new();
        let mut diag_2 = String::new();
        let mut diag_3 = String::new();
        let mut diag_4 = String::new();
        for i in 0..grid.len() - offset {
            diag_1.push(grid[i].chars().nth(i + offset).unwrap());
            diag_2.push(grid[i + offset].chars().nth(i).unwrap());
            diag_3.push(grid[i].chars().nth(grid.len() - i - offset - 1).unwrap());
            diag_4.push(grid[i + offset].chars().nth(grid.len() - i - 1).unwrap());
        }
        total += diag_1.matches("XMAS").count();
        total += diag_1.matches("SAMX").count();
        total += diag_2.matches("XMAS").count();
        total += diag_2.matches("SAMX").count();
        total += diag_3.matches("XMAS").count();
        total += diag_3.matches("SAMX").count();
        total += diag_4.matches("XMAS").count();
        total += diag_4.matches("SAMX").count();
    }
    total
}
