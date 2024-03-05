pub struct Row {
    pub title: String,
    pub value: String,
}

pub trait Display {
    fn format(&self, word_length: usize) -> String;
}

impl Display for Row {
    fn format(&self, word_length: usize) -> String {
        let needed_padding = word_length - self.title.len();
        let padding_whitespace = " ".repeat(needed_padding);

        format!(
            "{}{padding_whitespace}  {}",
            self.title,
            self.value
        )
    }
}
