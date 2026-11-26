pub fn get_diamond(c: char) -> Vec<String> {
    // if !c.is_alphabetic() && !c.is_uppercase() {
    //     return 
    // }
    let letters: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G',
        'H', 'I', 'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S', 'T', 'U',
        'V', 'W', 'X', 'Y', 'Z'
    ];

    let pos = letters.iter().position(|&x| x == c).unwrap() + 1;

    let height = &pos; 
    let width = &pos; 

    for i in 0..height {
        
    }


}
