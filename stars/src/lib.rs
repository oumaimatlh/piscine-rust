pub fn stars(n: u32) -> String {
    let mut new_str = String::new();
    for i in 1..=(2 as i32).pow(n) {
        new_str.push('*')
    }
    new_str

}