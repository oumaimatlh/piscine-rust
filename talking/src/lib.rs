pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let mut check_chars_maj = true;
    let mut has_letter = false;

    for c in text.chars() {
        if c.is_alphabetic() {
            has_letter = true;

            if c >= 'a' && c <= 'z' {
                check_chars_maj = false;
                break;
            }
        }
    }

    if check_chars_maj && has_letter && text.chars().last().unwrap() != '?' {
        return "There is no need to yell, calm down!";

    } else if check_chars_maj && has_letter && text.chars().last().unwrap() == '?' {
        return "Quiet, I am thinking!";

    } else if text.chars().last().unwrap() == '?' {
        return "Sure.";

    } else {
        return "Interesting";
    }
}