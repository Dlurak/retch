pub struct Row {
    pub title: String,
    pub value: String,
}

pub trait Display {
    fn format(&self) -> String;
}

impl Display for Row {
    fn format(&self) -> String {
        format!("{}: {}", self.title, self.value)
    }
}
