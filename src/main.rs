mod row;
mod utils;

use utils::{
    Information,
    get_information,
    Artwork,
    get_artwork
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
    };

    println!("{}", section.format(0));
    println!("{}", get_artwork(&Artwork::Tux).join("\n"));
}
