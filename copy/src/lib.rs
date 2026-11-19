pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut v : Vec<String> = vec![];
    let c  : Vec<&str> = a.split(" ").collect() ;
    for i in &c {
        v.push(i.parse::<f64>().unwrap().exp().to_string());
    }
    (a, v.join(" "))

}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut rVec: Vec<f64> = vec![];

    for v in &b {
        rVec.push((v.abs() as f64).ln());
    }
    (b, rVec)
}
