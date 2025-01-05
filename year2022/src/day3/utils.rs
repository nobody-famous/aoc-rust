use std::collections::HashSet;

pub fn get_char_set(s: &str) -> HashSet<char> {
    let mut chars = HashSet::new();

    for ch in s.chars() {
        chars.insert(ch);
    }

    chars
}

pub fn priority(ch: char) -> u32 {
    let lc_a = 'a' as u32;
    let uc_a = 'A' as u32;

    match ch {
        'a'..='z' => ch as u32 - lc_a + 1,
        'A'..='Z' => ch as u32 - uc_a + 26 + 1,
        _ => 0,
    }
}
