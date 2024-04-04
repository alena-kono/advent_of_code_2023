use std::{env, error::Error};

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Option<Config>, &str> {
        if args.len() < 2 {
            return Ok(None);
        }

        Ok(Some(Config {
            file_path: args[1].clone(),
        }))
    }
}

pub fn parse_config_from_cli() -> Result<Option<Config>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    Ok(Config::build(&args)?)
}
