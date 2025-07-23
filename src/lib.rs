use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub fpath: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // skip program name, first arg returned by env::args() by default
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get query")
        };

        let fpath = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get file path")
        };

        Ok(Self {
            query: query,
            fpath: fpath,
            case_sensitive: env::var("CASE_SENSITIVE").is_ok()
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str, case: bool) -> Vec<&'a str> {
    let query = if case {String::from(query.to_lowercase())} else {String::from(query)};

    content.lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.fpath)?;
    let results = search(&config.query, &content, config.case_sensitive);

    if results.len() > 0 {
        println!("{} results found matching \"{}\":", results.len(), config.query);
        results.iter().for_each(|result| {
            println!("{result}");
        });
    } else {
        println!("No results found for \"{}\"", config.query);
    }

    Ok(())
}
