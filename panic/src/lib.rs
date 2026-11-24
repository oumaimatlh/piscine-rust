use std::fs::File;
pub fn open_file(s: &str) -> File {
    let result = File::open(s).unwrap();
    result

    /*
        Second Method ; 
        let result = File::open(s);  //return Result<T, E>
        match result {
            Ok(v) => {
                v 
            }
            Err(err) => {
                panic!("{}", err)
            }
        }        
    
    */
}
