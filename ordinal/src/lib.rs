pub fn num_to_ordinal(x: u32) -> String {
    let parse_st: String = x.to_string(); 

    let last_number = parse_st.chars().nth(parse_st.len() - 1).unwrap(); 
   let ch_number = if parse_st.len() >= 2 {
        &parse_st[parse_st.len() - 2..]
        } else {
            &parse_st[..]
        };
    if last_number == '1' && ch_number != "11"{
        format!("{}{}", parse_st, "st".to_string())
    }else if  last_number == '2' && ch_number != "12" {
        format!("{}{}", parse_st, "nd".to_string())
    }else if last_number == '3' && ch_number != "13" {
        format!("{}{}", parse_st, "rd".to_string())
    }else {
        format!("{}{}", parse_st, "th".to_string())

    }
}
/*
- 1st → one
- 2nd → two
- 3rd → three
- 4th → four
- 5th → five
- 6th → six
- 7th → seven
- 8th → eight
- 9th → nine
- 10th → ten

*/