use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for result in results {
        println!("{result}");
    }

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: minigrep [query] [file]");
        }

        let query: String = args[1].to_string();
        let file_name: String = args[2].to_string();

        Ok(Config {
            query,
            file_name,
            case_sensitive: true,
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "COOL";
        let contents = "\
HI
MOM
COOL MOM
HI MOM THIS IS SOO COOL
cool";

        assert_eq!(
            vec!["COOL MOM", "HI MOM THIS IS SOO COOL"],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "cOoL";
        let contents = "\
HI
MOM
cool MOM
HI MOM THIS IS SOO COOL";

        assert_eq!(
            vec!["cool MOM", "HI MOM THIS IS SOO COOL"],
            search_case_insensitive(query, contents)
        );
    }
}
