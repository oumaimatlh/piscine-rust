pub fn pig_latin(text: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let chars: Vec<char> = text.chars().collect();

    if vowels.contains(&chars[0]) {
        return format!("{}ay", text);
    }

    let mut index = 0;

    for i in 0..chars.len() {
        if vowels.contains(&chars[i]) {
            index = i;
            break;
        }
    }

    if chars.len() >= 3 && chars[1] == 'q' && chars[2] == 'u' {
        index = 3;
    }

    let first = chars[index..].iter().collect::<String>();
    let last = chars[..index].iter().collect::<String>();

    format!("{}{}ay", first, last)
}