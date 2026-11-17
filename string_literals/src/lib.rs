pub fn is_empty(v: &str) -> bool {

    return v.len() == 0 
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars(){
        if c as u32 > 127 {
            return false 
        }
    }
    return true 
}

pub fn contains(v: &str, pat: &str) -> bool {
    let mut i = 0;
    loop {
        if i > v.len(){
            break;
        }
        if  i + pat.len() < v.len() && &v[i..i+pat.len()] == pat {
            return true 
        }
        i += 1;
    }
    return false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
        (&v[0..index], &v[index..v.len()])
}

pub fn find(v: &str, pat: char) -> usize {
    let mut r: usize = 0;
    for (i,c) in v.char_indices() {
        if c == pat {
            r = i;
            break;
        }
    }
    r
}