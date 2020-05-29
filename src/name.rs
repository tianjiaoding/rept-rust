pub fn valid_name_len(name: &str) -> bool {
    name.len() <= 20 && name.len() >= 1
}

pub fn valid_name_chars(name: &str) -> bool {
    name.chars().all(|c| {
        (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || (c >= '0' && c <= '9')
    })
}
