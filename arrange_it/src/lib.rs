pub fn arrange_phrase(phrase: &str) -> String {
    let mut r = String::new();
    let a: Vec<&str> = phrase.split_whitespace().collect();

    let mut i = 1;

    loop {
        if i > a.len() {
            break;
        }

        for v in &a {
            let mut newvalue = v.to_string();

            if v.contains((i as u8 + b'0') as char) {
                newvalue.remove(v.find((i as u8 + b'0') as char).unwrap());
                r.push_str(&newvalue);
                r.push_str(" ");
                i += 1;
                break;
            }
        }
    }

    r.trim()
}