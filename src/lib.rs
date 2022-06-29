use std::{fs, error::Error, env};

pub struct Config {
   pub query: String,
   pub filename: String,
   pub case_sensitive: bool
}

impl Config {
   pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

    //Skipping the first argument
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name "),
        };

    let case_sensitive = env::var("CASE_SENSITIVE").is_err();


     Ok(Config { query, filename, case_sensitive })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        println!("With text: {}", contents);


        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("Result: {}", line);
        }
    
         Ok(())
    
    }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\n Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
