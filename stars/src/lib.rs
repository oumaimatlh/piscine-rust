pub fn stars(n: u32) -> String {
    let mut new_str = String::new();
    for i in 1..n {
        for j in 0..=i {
            new_str.push('*')
        }
    }
    new_str
}