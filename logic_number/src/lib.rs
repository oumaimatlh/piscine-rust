pub fn number_logic(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let st = num.to_string();
    let len: u32= st.len() as u32;
    let mut sum:u32= 0;

    for c in st.chars(){
        sum += (c as u32 - '0' as u32).pow(len);
    }
    if sum == num {
        return true;
    }
    return false;
}
