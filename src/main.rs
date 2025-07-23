use std::{process, env};
use grep::{Config, run};

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
    if let Err(error) = run(config) {
        eprintln!("Application task error: {error}");
        process::exit(1);
    } 

}

#[cfg(test)]
mod test {
    use super::*;
    use grep::search;

    #[test]
    fn multiple_results() {
        let query = "black";
        let content = "\
In chess there exists two colors:
white and black
the white signifies the player who starts
black plays after white";

        assert_eq!(vec!["white and black", "black plays after white"], search(query, content, false));
    }

    #[test]
    fn insufficient_args() {
        let args = vec![String::from("binary_path")];
        let args2 = vec![String::from("binary_path"), String::from("query")];

        assert!(Config::build(args.into_iter()).is_err());
        assert!(Config::build(args2.into_iter()).is_err());
    }

    #[test]
    fn impossible_file() {
        let mut args = vec![String::new(), String::from("query"), String::from("data/test.ini")];
        assert!(run(Config::build(args.clone().into_iter()).unwrap()).is_err());

        args[2].replace_range(9..12, "jpg");
        assert!(run(Config::build(args.clone().into_iter()).unwrap()).is_err());

        args[2].replace_range(9..12, "exe");
        assert!(run(Config::build(args.clone().into_iter()).unwrap()).is_err());
    }
}