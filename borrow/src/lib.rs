pub fn str_len(s: &str) -> i32 {

    let mut count = 0;
    for _ in s.chars()  {
        count += 1;
    }
    return count
}
