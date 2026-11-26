pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut r = String::new();
    let mut v = vec![];
    
    let mut subarr:Vec<char> = vec![];

    for c in message.chars() {
        if i == subarr.len() {
            v.push(subarr.clone());
            subarr.clear();
        }

        subarr.push(c);
    }
    if subarr.len() != i {
        let c = i - subarr.len();
        for q in 1..c {
            subarr.push(' ');
        }
    }
    v.push(subarr.clone());

    for j in 0..i {
        for vec in &v {
            if j < vec.len(){
                r.push(vec[j]);
            }
        }

    }
    r.trim().to_string()
    
}


