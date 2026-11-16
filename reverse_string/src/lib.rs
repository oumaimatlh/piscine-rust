pub fn rev_str(input: &str) -> String {
    let mut result =  String::new();
    for c in input.chars() {
        result = c.to_string() + &result;

    }
    result
}
