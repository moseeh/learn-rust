use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag<'a> {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag<'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Flag {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand, flag.long_hand), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if argv.len() < 2 {
            return Err("Not enough arguments".to_string());
        }

        // Find the matching flag
        for ((short, long), callback) in &self.flags {
            if input == short || input == long {
                // Execute the callback with the first two arguments
                return callback(argv[0], argv[1]).map_err(|e| e.to_string());
            }
        }

        Err(format!("Flag {} not found", input))
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_float = a.parse::<f64>()?;
    let b_float = b.parse::<f64>()?;
    
    Ok((a_float / b_float).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_float = a.parse::<f64>()?;
    let b_float = b.parse::<f64>()?;
    
    Ok((a_float % b_float).to_string())
}