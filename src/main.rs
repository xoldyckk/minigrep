use minigrep::Config;
use std::{env, process};

fn main() {
    // iterator over the provided arguments, collected into a vector of strings
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|error| {
        // eprintln is used to print errors to the standard error, errors
        // should always be printed to standard error and not standard output
        eprintln!("Problem parsing the arguments: {error}");
        // exiting with non-zero exit code signals the process that the
        // program exited with an error state
        process::exit(1);
    });

    // here we're destructing the error if it is returned as the value for Result
    // enum returned from the function, if let syntax is used instead of
    // unwrap_or_else because we have no use of the result in case of success
    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
