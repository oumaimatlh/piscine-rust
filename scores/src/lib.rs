use std::collections::HashMap;

pub fn score(s: &str) -> u64 {
    let mut hash_map = HashMap::new();
    let mut count:u64 = 0; 
    hash_map.insert(1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T',
                        'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't']);

    hash_map.insert(2, vec!['D', 'G', 'd', 'g']);
    hash_map.insert(3, vec!['B', 'C', 'M', 'P', 'b', 'c', 'm', 'p']);
    hash_map.insert(4, vec!['F', 'H', 'V', 'W', 'Y', 'f', 'h', 'v', 'w', 'y']);
    hash_map.insert(5, vec!['K', 'k']);
    hash_map.insert(8, vec!['J', 'X', 'j', 'x']);
    hash_map.insert(10, vec!['Q', 'Z', 'q', 'z']);

    for c in s.chars(){
        for (key, value) in &hash_map{
            if value.contains(&c) {
                count += *key;
                break;
            }
        }
    }
    count 
}