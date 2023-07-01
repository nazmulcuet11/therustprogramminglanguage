use minigrep;
use std::{env, process};

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!(
            "Problem parsing arguments: {}.\nargs: {:#?}",
            err,
            env::args()
        );
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("App error: {e}");
        process::exit(1);
    }
}
