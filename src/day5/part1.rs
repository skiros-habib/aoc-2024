use std::collections::{BTreeMap, BTreeSet};

use super::parse_input;

fn valid(update: &[u32], rules: &BTreeMap<u32, BTreeSet<u32>>) -> bool {
    let mut no: BTreeSet<u32> = BTreeSet::new();
    for page in update {
        if no.contains(&page) {
            return false;
        }
        if rules.contains_key(&page) {
            no = &no | rules.get(&page).expect("I just checked if its there");
        }
    }
    true
}

pub fn solve(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let mut middle: usize = 0;
    updates.iter().for_each(|update| {
        if valid(&update, &rules) {
            middle += update[(update.len() - 1) / 2] as usize;
        }
    });
    middle
}
