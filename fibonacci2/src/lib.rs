pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut i = 1;

    let mut prev = 0;
    let mut curr = 1;

    let mut nv = 0;

    loop {
        if i >= n {
            break;
        }
        nv = prev + curr;
        prev = curr;
        curr = nv;
        i += 1
    }
    curr
}
