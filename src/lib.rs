use std::fs;
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("no hay suficientes argumentos");
        }

        let query = &args[1];
        let filename = &args[2];
    
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenido = fs::read_to_string(config.filename)?;
    println!("Con texto: \n{}", contenido);

    Ok(())
}

