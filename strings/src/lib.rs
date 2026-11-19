pub fn char_length(s: &str) -> usize {
    let mut count = 0;
    for v in s.chars() {
        count += 1
    }
    count
}