mod row;
mod utils;

use utils::{
    Information,
    get_information
};
use crate::row::{
    Display,
    Row
};

fn main() {
    let pairs = vec![
        ("Hostname", Information::Hostname),
        ("Window Manager", Information::WindowManager),
        ("Session type", Information::SessionType),
        ("User", Information::User),
    ];

    let rows: Vec<_> = pairs
        .iter()
        .filter_map(|(title, information)| get_information(information)
            .map(|inf| Row {
                title: title.to_string(),
                value: inf
            })
        )
        .collect();

    let title_width = rows
        .iter()
        .map(|row| row.title.len())
        .max()
        .unwrap_or(0);

    for row in rows {
        println!("{}", row.format(title_width))
    }
}
