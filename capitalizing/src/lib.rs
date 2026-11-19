pub fn capitalize_first(input: &str) -> String {
    let mut result = String::new();
    for (i, v) in input.char_indices() {
        let mut st =  v.to_string();
        if i == 0 && v >= 'a' && v <= 'z'  {
            st = ((v as u8 - 32)as char).to_string(); 
        }
        let r :&str = &st;
        result.push_str(r);
    }
    result
}

pub fn title_case(input: &str) -> String {
    let mut split = input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    for value in &mut split {
        *value = capitalize_first(value);
    }
    split.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut new_string = String::new();

     for (i, v) in input.char_indices() {
        if v == ' ' {
           new_string.push_str(" ")
        }
        if v >= 'a' && v <= 'z' {
            let st = ((v as u8 - 32)as char).to_string();
            new_string.push_str(&st);
        }
        if  v >= 'A' && v <= 'Z' {
            let st = ((v as u8 + 32)as char).to_string();
            new_string.push_str(&st);
        }
     }
    new_string
}