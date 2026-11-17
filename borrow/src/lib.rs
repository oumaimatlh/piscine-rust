pub fn str_len(s: &str) -> String{

    let mut count = 0;
    for c in s.chars()  {
        count += 1;
    }
    return count.to_string()

}
