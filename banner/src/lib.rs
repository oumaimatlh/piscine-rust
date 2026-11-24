use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().nth(0).unwrap()),
            long_hand: format!("--{}", name), 
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let callback = self.flags.get(input).unwrap();
        let result =  callback(argv[0], argv[1]);
        match result {
            Ok(v)=> {
                Result::Ok(v)
            }
            Err(err)=> {
                Result::Err(err.to_string())

            }
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let  parse_a = a.parse::<f64>();
    let  float_a:f64;
    match parse_a {
        Ok(value) => {float_a = value}
        Err(err) => return Result::Err(err)
    }

    let parse_b = b.parse::<f64>();
    let  float_b:f64;

    match parse_b {
        Ok(value) => {float_b = value }
        Err(err) => return Result::Err(err)
    }
    Result::Ok((float_a / float_b).to_string())

}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
     let  parse_a = a.parse::<f64>();
    let  float_a:f64;
    match parse_a {
        Ok(value) => {float_a = value}
        Err(err) => return Result::Err(err)
    }

    let parse_b = b.parse::<f64>();
    let  float_b:f64;

    match parse_b {
        Ok(value) => {float_b = value }
        Err(err) => return Result::Err(err)
    }
    Result::Ok((float_a % float_b).to_string())
}

