pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut last_index:Option<usize> = Option::None; 

    for i in 0..array.len() {
        if array[i] == key {
            last_index = Option::Some(i);
        }
    }
    
    last_index
}
