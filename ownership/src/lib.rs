pub fn first_subword(mut s: String) -> String {
    let mut r = String::new();
    for (i, c) in s.char_indices() {

        if (c as u32 >= 65 && c as u32 <= 90  && i != 0) ||  c == '_' {
            break;
        }
        r.push(c);
    }
    r

}
