use crate::utils::strings::fill_to_len;

pub trait Display {
    fn format(&self, word_length: usize) -> String;
    fn len(&self, word_length: usize) -> usize {
        self.format(word_length).len()
    }
}

pub struct Header {
    pub value: String,
}

impl Display for Header {
    fn format(&self, total_len: usize) -> String {
        let side_len = (total_len - self.value.len()) / 2;
        let side = "─".repeat(side_len);
        format!("{side}{}{side}", self.value)
    }
}

pub struct Row {
    pub title: String,
    pub value: String,
}

impl Display for Row {
    fn format(&self, word_length: usize) -> String {
        format!("{}  {}", fill_to_len(&self.title, word_length), self.value)
    }
}

pub struct Section {
    pub header: Option<Header>,
    pub rows: Vec<Row>,
}

impl Display for Section {
    fn format(&self, _total_len: usize) -> String {
        let title_width = self.rows.iter().map(|r| r.title.len()).max().unwrap_or(0);
        let total_width = self
            .rows
            .iter()
            .map(|r| r.len(title_width))
            .max()
            .unwrap_or(0);

        let rows: Vec<_> = self
            .rows
            .iter()
            .map(|r| format!("│ {} │", fill_to_len(&r.format(title_width), total_width)))
            .collect();

        let vertical_base = match &self.header {
            Some(h) => h.format(total_width + 2),
            None => "─".repeat(total_width + 2),
        };
        let top_border = format!("┌{}┐", vertical_base);
        let bottom_border = format!("└{}┘", vertical_base);

        format!("{}\n{}\n{}", top_border, rows.join("\n"), bottom_border)
    }
}
