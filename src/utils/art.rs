pub enum Artwork {
    Tux,
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
    }
}
