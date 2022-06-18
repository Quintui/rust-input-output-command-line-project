use std::{env, process};

use minigrep::Config;


fn main() {
    let args:  Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Something went wrong {}", err);
        process::exit(1)
    });

    println!("In file {}", config.filename);
    println!("File query, '{}'", config.query);
 

    if let Err(e) = Config::run(config) {
            println!("Application error: {}", e);
            process::exit(1)
    }

   
        
    
}


