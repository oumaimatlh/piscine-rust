pub fn is_permutation(s1: &str, s2: &str) -> bool {
    for (i, c) in s1.char_indices() {
        if c != s2.chars().nth(i).unwrap() {
            return false;
        }
    }
    return true; 
}
