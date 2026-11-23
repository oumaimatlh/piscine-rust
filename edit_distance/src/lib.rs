pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut count: usize = 0;
    if source.len() > target.len() {
        count += source.len() - target.len();
    }else if source.len() < target.len() {
        count += target.len() - source.len();

    }
    
    for (i, v) in target.char_indices(){
        if i < source.len() && v != source.chars().nth(i).unwrap(){
            count += 1
        }
    
    }
    
    count
}
