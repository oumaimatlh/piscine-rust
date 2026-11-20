use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut vec :Vec<i32> = vec![];
     
    for (key, value) in &h {
        vec.push(*value);
    }

    let mut bigger = vec[0];
    for n in vec {
        if n >=  bigger {
            bigger = n; 
        }
    }

    bigger
}
