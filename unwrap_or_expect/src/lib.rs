pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
        match security_level {
            Security::Unknown => {
                match server {
                    Ok(url) => {
                        String::from(url)
                    }
                    Err(error) => {
                        server.unwrap().to_string()
                    }
                }

            }
            Security::Message => {
                match server {
                    Ok(url) => {
                        String::from(url)
                    }
                    Err(error) => {
                        panic!("ERROR: program stops")
                    }
                }
            }

            Security::Warning => {
                match server {
                    Ok(url) => {
                        String::from(url)
                    }
                    Err(error) => {
                        String::from("WARNING: check the server")
                    }
                }
            }
            Security::NotFound => {
                match server {
                    Ok(url) => {
                        String::from(url)
                    }
                    Err(error) => {
                        format!("Not found: {}", error)
                    }
                }

            }
            Security::UnexpectedUrl => {
                 match server {
                    Ok(url) => {
                        panic!("{}", url)
                    }
                    Err(error) => {
                        String::from(error)
                    }
                }
            }
         
            
            
        }
}