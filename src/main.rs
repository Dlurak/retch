mod row;
mod utils;

use crate::row::{Display, Header, Row, Section};
use std::env;
use utils::{get_artwork, get_information, strings, vecs, Artwork, Information};

fn main() {
    let pairs = [
        ("Window Manager", Information::WindowManager),
        ("Shell", Information::Shell),
        ("Memory", Information::Memory),
    ];

    let rows: Vec<_> = pairs
        .iter()
        .filter_map(|(title, information)| {
            get_information(information).map(|inf| Row {
                title: title.to_string(),
                value: inf,
            })
        })
        .collect();

    let section = Section {
        header: Some(Header {
            value: format!(
                "{}@{}",
                get_information(&Information::User).unwrap_or_default(),
                get_information(&Information::Hostname).unwrap_or_default()
            ),
        }),
        rows,
    }
    .format(0);

    let section_row_amount = section.split('\n').collect::<Vec<_>>().len();

    let art_name = env::args().nth(1).unwrap_or("".to_string()).to_lowercase();
    let art_name = art_name.as_str();
    let art_enum = match art_name {
        "tux" => Artwork::Tux,
        "none" => Artwork::None,
        _ => Artwork::Tux,
    };

    let art = strings::fill_vec_to_len(vecs::fill_til_length(
        get_artwork(&art_enum),
        section_row_amount,
        "",
    ));

    let output_lines = vecs::create_pairs(
        art.iter().map(String::as_str).collect::<Vec<_>>(),
        section.split('\n').collect(),
    );

    for line in output_lines {
        let first = line.0.unwrap_or("");
        let second = line.1.unwrap_or("");
        println!("{first}  {second}");
    }
}
