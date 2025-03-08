mod art;
mod bordered_data;
mod information;
mod memory;

use crate::{bordered_data::BorderedData, information::Information};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let pairs = [
        ("Window Manager", Information::WindowManager),
        ("Shell", Information::Shell),
        ("Memory", Information::Memory),
    ];

    let data: HashMap<_, _> = pairs
        .iter()
        .filter_map(|(title, information)| {
            let value = information.get_information()?;
            Some((title.to_string(), value))
        })
        .collect();

    let art = std::env::args()
        .nth(1)
        .and_then(|s| art::Art::file_path(s.into()))
        .unwrap_or_default();
    let art_lines = art.block(data.len() + 2);

    let header = format!(
        "{}@{}",
        Information::User.get_information().unwrap_or_default(),
        Information::Hostname.get_information().unwrap_or_default()
    );
    let section = BorderedData::new(header, data).to_string();

    let output_lines = art_lines.zip_longest(section.lines());

    for line in output_lines {
        let (first, second) = line.left_and_right();
        let first = first.unwrap_or_default();
        let second = second.unwrap_or_default();
        println!("{first}  {second}");
    }
}
