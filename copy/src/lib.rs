pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), ((c).abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
   let mut r : Vec<String> = vec![];
   let vec : Vec<&str> = a.split(" ").collect();

   for v in &vec {
      r.push(v.parse::<f64>().unwrap().exp().to_string());
   }
   (vec.join(" "), r.join(" "))


}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut r : Vec<f64> = vec![];

    for mut v in &b {
    
      r.push(((*v).abs() as f64).ln());
    }
   (b, r)
}
