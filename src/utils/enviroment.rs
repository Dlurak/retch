use std::env;

use crate::row;

fn pair_to_row(pair: &(&str, &str)) -> Option<row::Row> {
    let (title, key) = *pair;
    env::var(key)
        .ok()
        .map(|value| row::Row {
            title: title.to_string(),
            value,
        })
}

pub fn pairs_to_rows(pairs: Vec<(&str, &str)>) -> Vec<row::Row> {
    pairs.iter().filter_map(|p| pair_to_row(p)).collect()
}
