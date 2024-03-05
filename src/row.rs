pub struct Header {
    pub value: String
}

pub struct Row {
    pub title: String,
    pub value: String,
}

pub trait Display {
    fn format(&self, word_length: usize) -> String;
    fn len(&self, word_length: usize) -> usize {
        self.format(word_length).len()
    }
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

impl Display for Header {
    fn format(&self, total_len: usize) -> String {
        let side_len = (total_len - self.value.len()) / 2;
        let side = "-".repeat(side_len);
        format!("{side}{}{side}", self.value)
    }
}
