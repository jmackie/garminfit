/// "0x80" -> 128
pub(crate) fn parse_hex(s: &str) -> u64 {
    u64::from_str_radix(&s[2..], 16).expect("well formed hex")
}

/// Test if all cells in a row are empty.
pub(crate) fn all_empty(row: &[calamine::DataType]) -> bool {
    for cell in row {
        if *cell != calamine::DataType::Empty {
            return false
        }
    }
    true
}

/// "IS SHOUTING" == true
pub(crate) fn is_shouting(s: &String) -> bool {
    s.chars().all(|c| c.is_whitespace() || c.is_uppercase())
}

/// Uppercase the first letter of a string.
pub(crate) fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
