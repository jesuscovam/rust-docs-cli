use std::fs;
use std::error::Error;
use std::env;
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("no hay suficientes argumentos");
        }

        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenido = fs::read_to_string(config.filename)?;
    
    let resultado = if config.case_sensitive {
        search(config.query, &contenido) 
    } else {
        search_case_insensitive(config.query, &contenido)
    };

    for line in resultado {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut resultado = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            resultado.push(line)
        }
    }

    resultado
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut resultado = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()){
            resultado.push(line)
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
