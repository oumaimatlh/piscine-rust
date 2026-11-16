pub fn factorial(num:u64) -> u64 {
    if num == 0 || num == 1 {
        return 1
    }
    let mut count = 1;
    let mut result = 1;
    loop {
        if count > num {
            break
        }
        result *= count;
        count += 1 ;
    }
    return result

}