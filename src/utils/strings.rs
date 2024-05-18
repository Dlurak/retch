pub fn fill_to_len(string: &str, desired_len: &usize) -> String {
    let original_len = string.len();
    if original_len >= *desired_len {
        return string.to_string();
    }
    string.to_owned() + &" ".repeat(desired_len - original_len)
}

pub fn fill_vec_to_len(strings: Vec<&str>) -> Vec<String> {
    let max_width = strings.iter().map(|str| str.len()).max().unwrap_or(0);

    strings
        .iter()
        .map(|str| fill_to_len(str, &max_width))
        .collect::<Vec<_>>()
}
