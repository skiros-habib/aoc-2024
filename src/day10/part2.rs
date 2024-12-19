use std::collections::HashSet;

use super::{adjacent_point, s_parse};

pub fn solve(input: &str) -> usize {
    let info = s_parse(input);
    let possible_trailheads = &info[0];
    let mut total = 0;
    for (x_0, y_0) in possible_trailheads {
        let mut routes = HashSet::new();
        let level_1 = info[1]
            .iter()
            .filter(|(x, y)| adjacent_point(*x_0, *y_0, *x, *y));
        for (x_1, y_1) in level_1 {
            let level_2 = info[2]
                .iter()
                .filter(|(x, y)| adjacent_point(*x_1, *y_1, *x, *y));
            for (x_2, y_2) in level_2 {
                let level_3 = info[3]
                    .iter()
                    .filter(|(x, y)| adjacent_point(*x_2, *y_2, *x, *y));
                for (x_3, y_3) in level_3 {
                    let level_4 = info[4]
                        .iter()
                        .filter(|(x, y)| adjacent_point(*x_3, *y_3, *x, *y));
                    for (x_4, y_4) in level_4 {
                        let level_5 = info[5]
                            .iter()
                            .filter(|(x, y)| adjacent_point(*x_4, *y_4, *x, *y));
                        for (x_5, y_5) in level_5 {
                            let level_6 = info[6]
                                .iter()
                                .filter(|(x, y)| adjacent_point(*x_5, *y_5, *x, *y));
                            for (x_6, y_6) in level_6 {
                                let level_7 = info[7]
                                    .iter()
                                    .filter(|(x, y)| adjacent_point(*x_6, *y_6, *x, *y));
                                for (x_7, y_7) in level_7 {
                                    let level_8 = info[8]
                                        .iter()
                                        .filter(|(x, y)| adjacent_point(*x_7, *y_7, *x, *y));
                                    for (x_8, y_8) in level_8 {
                                        let level_9 = info[9]
                                            .iter()
                                            .filter(|(x, y)| adjacent_point(*x_8, *y_8, *x, *y));
                                        for (x_9, y_9) in level_9 {
                                            let route = format!("({},{}) ({},{}) ({},{}) ({},{}) ({},{}) ({},{}) ({},{}) ({},{}) ({},{}) ({},{})",
                                                     x_0, y_0, x_1, y_1, x_2, y_2, x_3, y_3, x_4, y_4, x_5, y_5, x_6, y_6, x_7, y_7, x_8, y_8, x_9, y_9);
                                            routes.insert(route);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        total += routes.len();
    }
    total
}
