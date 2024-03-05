mod row;
mod utils;

use utils::{
    Information,
    get_information
};
use crate::row::{
    Display,
    Row,
    Header
};

fn main() {
    let pairs = vec![
        ("Window Manager", Information::WindowManager),
        ("Session type", Information::SessionType),
        ("Shell", Information::Shell),
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

    let total_width = rows
        .iter()
        .map(|row| row.len(title_width))
        .max()
        .unwrap_or(0);

    let header = Header {
       value: format!(
          "{}@{}",
          get_information(&Information::User).unwrap_or(String::new()),
          get_information(&Information::Hostname).unwrap_or(String::new()),
       )
    };

    println!("{}", header.format(total_width));
    for row in rows {
        println!("{}", row.format(title_width))
    }
}
