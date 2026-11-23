pub fn check_ms(message: &str) -> Result<&str, &str> {


    let split = message.split(" ").collect::<Vec<&str>>(); 
    if split.contains(&"stupid")  || message.len() == 0 {
        return Result::Err("ERROR: illegal")
    }
    Result::Ok(message)
}
