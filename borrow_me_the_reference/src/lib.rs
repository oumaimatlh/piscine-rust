pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '-' {
            result.pop();
        } else if chars[i] == '+' {
            let mut count = 0;

            while i < chars.len() && chars[i] == '+' {
                count += 1;
                i += 1;
            }

            i += count;
            continue;
        } else {
            result.push(chars[i]);
        }

        i += 1;
    }

    s.clear();
    s.push_str(&result);
}

pub fn do_operations(v: &mut [String]) {
    for op in v {
        let operator = op.find(['+', '-']).unwrap();
        let (a, b) = op.split_at(operator);

        let x = a.parse::<i32>().unwrap();
        let y = b[1..].parse::<i32>().unwrap();

        let result = if &b[..1] == "-" {
            x - y
        } else {
            x + y
        };

        *op = result.to_string();
    }
}