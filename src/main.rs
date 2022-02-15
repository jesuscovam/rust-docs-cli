use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problema parsiando argumentos: {}", err);
        process::exit(1);
    });

    println!("Buscando {}", config.query);
    println!("En archivo {}", config.filename);

    if let Err(e) = run(config) {
        println!("Error en la aplicación: {}", e);
        process::exit(1);
    }
}


