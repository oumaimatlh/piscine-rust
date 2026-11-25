pub fn rotate(input: &str, key: i8) -> String {
    let mut new_str = String::new();

    let letters: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g',
        'h', 'i', 'j', 'k', 'l', 'm', 'n',
        'o', 'p', 'q', 'r', 's', 't', 'u',
        'v', 'w', 'x', 'y', 'z'
    ];

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let ch = letters.iter().position(|&x| x == c.to_ascii_lowercase()).unwrap();
            let mut index = ch as i32 + key as i32;

            if index < 0 {
                index += 26;
            } else if index > 25 {
                index -= 26;
            }

            let index = index as usize;
            let mut new_c = letters[index];

            if c.is_uppercase() {
                new_c = new_c.to_ascii_uppercase();
            }

            new_str.push(new_c);
        } else {
            new_str.push(c);
        }
    }

    new_str
}