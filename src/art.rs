pub struct Art(String);

struct FillDefault<T: Iterator<Item = D>, D: Default>(usize, T);

impl<T: Iterator<Item = D>, D: Default> FillDefault<T, D> {
    fn new(min_length: usize, iterator: T) -> Self {
        Self(min_length, iterator)
    }
}

impl<T: Iterator<Item = D>, D: Default> Iterator for FillDefault<T, D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        let is_too_short = self.0 > 0;
        self.0 = self.0.saturating_sub(1);
        self.1.next().or_else(|| is_too_short.then(D::default))
    }
}

impl Art {
    pub fn tux() -> Self {
        Self(include_str!("../art/tux").to_string())
    }

    pub fn file_path(path: std::path::PathBuf) -> Option<Self> {
        let s = std::fs::read_to_string(path).ok()?;
        Some(Self(s))
    }

    pub fn block(&self, min_lines: usize) -> impl Iterator<Item = String> + '_ {
        let lines: Vec<String> = self.0.lines().map(|line| line.to_string()).collect();
        let width = lines
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or_default();

        FillDefault::new(min_lines, lines.into_iter())
            .map(move |line| format!("{:<width$}", line, width = width))
    }
}

impl Default for Art {
    fn default() -> Self {
        Self::tux()
    }
}
