mod row;
mod utils;

use utils::{
    Information,
    get_information,
    Artwork,
    get_artwork,
    strings,
    vecs
};

use crate::row::{
    Display,
    Row,
    Header,
    Section
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

    let section = Section {
        header: Some(Header {
            value: format!(
                "{}@{}",
                get_information(&Information::User).unwrap_or(String::new()),
                get_information(&Information::Hostname).unwrap_or(String::new()),
            )
        }),
        rows 
    }.format(0);

    let section_row_amount = section
        .split("\n")
        .collect::<Vec<_>>()
        .len();

    let art = strings::fill_vec_to_len(
        vecs::fill_til_length(
            get_artwork(&Artwork::Tux),
            section_row_amount,
            ""
        )
    );

    let output_lines = vecs::create_pairs(
        art.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        section.split("\n").collect()
    );

    for line in output_lines {
        let first = line.0.unwrap_or("");
        let second = line.1.unwrap_or("");
        println!("{first}  {second}");
    }
}
