pub fn process_id_text(input: String) -> String {
    let s = input.trim().to_lowercase();
    let mut result = String::new();

    for c in s.chars() {
        match c {
            '\n' | '_' => continue,
            '-' | '.' | ',' | ' ' => result.push('_'),
            _ => result.push(c),
        }
    }

    result.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '_')
        .collect()
}