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
    let mut result=String::from("");
    let  mut i=0;
    let charf:Vec<char>=input.chars().collect();
    while i<charf.len(){
        if i==0||charf[i-1]==' '||charf[i-1].is_whitespace(){
            result.push(charf[i].to_ascii_uppercase());
        }else{
            result.push(charf[i]);
        }
        i+=1;
    }
    result
}


pub fn change_case(input: &str) -> String {
    let mut new_string = String::new();

     for v in input.chars() {
        if v == ' ' {
           new_string.push_str(" ");
        }else if v >= 'a' && v <= 'z' {
            let st = ((v as u8 - 32)as char).to_string();
            new_string.push_str(&st);
        }else if v >= 'A' && v <= 'Z' {
            let st = ((v as u8 + 32)as char).to_string();
            new_string.push_str(&st);
            continue;

        }else {new_string.push_str(&v.to_string());}
     }
    new_string
}