use std::collections::HashMap;
use std::fmt;

const HORIZONTAL: char = '─';

pub struct BorderedData {
    pub header: String,
    pub data: HashMap<String, String>,
}

impl BorderedData {
    pub fn new(header: String, data: HashMap<String, String>) -> Self {
        Self { header, data }
    }
}

impl fmt::Display for BorderedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key_width = self.data.keys().map(|k| k.len()).max().unwrap_or(0);
        let value_width = self.data.values().map(|v| v.len()).max().unwrap_or(0);
        let total_width = key_width + value_width + 3;

        let horizontal_border = HORIZONTAL.to_string().repeat(total_width);

        let filler_length = total_width.saturating_sub(self.header.len());
        let left_padding = filler_length / 2;
        let right_padding = filler_length - left_padding;
        writeln!(
            f,
            "┌{}{}{}┐",
            HORIZONTAL.to_string().repeat(left_padding),
            self.header,
            HORIZONTAL.to_string().repeat(right_padding)
        )?;

        for (title, value) in &self.data {
            writeln!(
                f,
                "│ {:<key_width$} {:<value_width$} │",
                title,
                value,
                key_width = key_width,
                value_width = value_width
            )?;
        }
        write!(f, "└{}┘", horizontal_border)
    }
}
