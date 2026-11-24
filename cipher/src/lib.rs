#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut  str_ciphered = String::new();
    for c in original.chars() {
        if c >= 'A' && c <= 'Z' {
            let n = 155 - (c as u8);
            str_ciphered.push(n as char);
            continue;
        }
        if c >= 'a' && c <= 'z' {
            let n = 219 - (c as u8);
            str_ciphered.push(n as char);
            continue;
        } 
        str_ciphered.push(c);
    }
     if ciphered == str_ciphered {
           return Result::Ok(());
        }
        let cipher_error = CipherError{expected: str_ciphered};
        return Result::Err(cipher_error);
}