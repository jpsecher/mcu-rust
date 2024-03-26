use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With the text:\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub fn add(left: u8, right: u8) -> u8 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_out() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn can_find_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick any three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
