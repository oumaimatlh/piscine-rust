pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = vec![];
    for value in &names {
        let mut s = String::new();
        let split: Vec<&str> = value.split(" ").collect();

        for i in 0..split.len() {
            s.push(split[i].chars().next().unwrap());
            s.push('.');

            if i == split.len() - 1 {
                continue;
            } 
            s.push(' ');

        }
        result.push(s)
    }
    result
}
