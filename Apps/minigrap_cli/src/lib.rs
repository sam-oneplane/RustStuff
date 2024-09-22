use std::{fs, env};
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error> > {

    let content = fs::read_to_string(config.filename)?; // ? : if err return Error from run() 
    let results = 
    if config.case_sensitive { 
        search(&config.query, &content)
    }else {
        search_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next()  {
            Some(arg) => arg,
            None => return Err("No query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file name string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { filename, query, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents
        .lines()
        .filter(|line| {line.contains(query)})
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn test_sensitive() {
        let query = "fast";
        let contents = "\
Rust:
fast, safe,
productive.
all three.";
        assert_eq!(vec!["fast, safe,"], search(query, contents));
    }

    #[test]
    fn test_insensitive() {
        let query = "prod";
        let contents = "\
Rust:
fast, safe,
Productive, hard.
all Three.";
        assert_eq!(vec!["Productive, hard."], search_insensitive(query, contents));
    }

}

