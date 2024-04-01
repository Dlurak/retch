pub enum Artwork {
    Tux,
    None,
}

pub fn get_artwork(name: &Artwork) -> Vec<&str> {
    match name {
        Artwork::Tux => vec![
            "     .--.",
            "    |o_o |",
            "    |:_/ |",
            "   //   \\ \\",
            "  (|     | )",
            " /'\\_   _/`\\",
            " \\___)=(___/",
        ],
        Artwork::None => vec![],
    }
}
