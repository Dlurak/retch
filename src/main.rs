use std::env;

/* struct Row<'a> {
    title: &'a str,
    value: &'a str,
} */

struct Row {
    title: String,
    value: String,
}

trait Display {
    fn format(&self) -> String;
}

impl Display for Row {
    fn format(&self) -> String {
        format!("{}: {}", self.title, self.value)
    }
}

fn pair_to_row(pair: &(&str, &str)) -> Option<Row> {
    let pair = *pair;
    let value = env::var(pair.1);

    match value {
        Ok(s) => Some(Row {
            title: pair.0.to_string(),
            value: s
        }),
        Err(_) => None
    }
}

fn pairs_to_rows(pairs: Vec<(&str, &str)>) -> Vec<Row> {
    pairs.iter().filter_map(|p| pair_to_row(p)).collect()
}

fn main() {
    let pairs = vec![
        ("Window Manager", "XDG_CURRENT_DESKTOP")
    ];
    let rows = pairs_to_rows(pairs);

    for row in rows {
        println!("{}", row.format())
    }
}
