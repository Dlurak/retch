mod row;
mod utils;

use utils::pairs_to_rows;
use crate::row::Display;

fn main() {
    let pairs = vec![
        ("Window Manager", "XDG_CURRENT_DESKTOP"),
        ("Session tpye", "XDG_SESSION_TYPE"),
        ("User", "USER"),
        ("Hostname", "HOSTNAME")
    ];
    let rows = pairs_to_rows(pairs);

    for row in rows {
        println!("{}", row.format())
    }
}
