use std::collections::HashMap; 

pub fn mean(list: &[i32]) -> f64 {
    let mut sum:i32 = 0;
    for v in list {
        sum += *v; 
    }
    let avg = sum as f64 / list.len() as f64; 
    avg 
}

pub fn median(list: &[i32]) -> i32 {
    let mut v = list.to_vec();
    v.sort();

    let n = v.len();

    if n % 2 != 0 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut h = HashMap::new();
    for i in list {
        let count = h.entry(i).or_insert(0); 
        *count += 1;
    }

    let mut c = 0;
    let mut result :i32 = 1 ;
    for (key, value) in &h {
        if c <= *value {
            c = *value;
            result = **key; 
        }
        
    }
    result 
}
