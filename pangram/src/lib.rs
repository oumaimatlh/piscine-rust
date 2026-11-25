use std::collections::HashMap; 

pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashMap::new();
    for c in s.chars(){
        if c.to_ascii_lowercase() >= 'a' && c.to_ascii_lowercase() <= 'z' {
            letters.insert(c.to_ascii_lowercase(), "exist"); 
        }
    }

    let mut count = 0; 

    for (key, value) in &letters {
        count += 1;
    }
    if count == 26 {
        return true;
    }
    false 
}
