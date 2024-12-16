use std::collections::{BTreeMap, BTreeSet};

use super::parse_input;

fn invalid_index(update: &[u32], rules: &BTreeMap<u32, BTreeSet<u32>>) -> Option<usize> {
    let mut no: BTreeSet<u32> = BTreeSet::new();
    for (i, page) in update.iter().enumerate() {
        if no.contains(&page) {
            return Some(i);
        }
        if rules.contains_key(&page) {
            no = &no | rules.get(&page).expect("I just checked if its there");
        }
    }
    None
}

pub fn solve(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let mut middle: usize = 0;
    for mut update in updates {
        if invalid_index(&update, &rules).is_none() {
            continue;
        }

        loop {
            let i = invalid_index(&update, &rules);
            if let Some(i) = i {
                let temp = update[i];
                update[i] = update[i - 1];
                update[i - 1] = temp;
            } else {
                break;
            }
        }
        middle += update[(update.len() - 1) / 2] as usize;
    }
    middle
}
