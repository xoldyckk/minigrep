use std::{env, error::Error, fs};

pub struct Config {
    file_path: String,
    ignore_case: bool,
    query: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // ignore the first value as it is always the name of the program
        // and we have no use of it here
        args.next();

        // string to search the file for
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        // path address to the file to search in for the query
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };

        // determines whether the environment variabled named IGNORE_CASE
        // is provided by the user, if not provided ignore_case defaults to false
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            file_path,
            ignore_case,
            query,
        })
    }
}

// dyn is used when we're referring to traits inside generics, here
// it means any error which implements the Error trait can be
// returned from this function inside a Box
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the contents of file
    let file_contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search_case_sensitive(&config.query, &file_contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// here lifetime 'a tells the rust compiler that the return value of this
// function will live as long as the contents argument lives
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterates over the lines in contents, filters the lines that
    // contain query string, pushes them to results vector
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    // iterates over the lines in contents, filters the lines that
    // contain query string, pushes them to results vector
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let file_contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, file_contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let file_contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, file_contents)
        )
    }
}
