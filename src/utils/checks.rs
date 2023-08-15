pub fn is_hex_string(input: &str) -> bool {
    input.chars().all(|c| c.is_digit(16))
}

pub fn is_object_id(input: &str) -> bool {
    input.len() == 24 && is_hex_string(input)
}