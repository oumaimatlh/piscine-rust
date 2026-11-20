use std::collections::HashMap; 

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut h = HashMap::new();
    let mut h1 = HashMap::new();

    for c in s1.chars() {
        let mut count = h.entry(c).or_insert(0);
        *count += 1 
    }

    for c in s2.chars() {
        let mut count = h1.entry(c).or_insert(0);
        *count += 1 
    }
    if h == h1 {
        return true; 
    }
    false
}
