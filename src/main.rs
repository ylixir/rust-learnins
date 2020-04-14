use std::{ env, process };

use rust_learnins::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    let config = Config::new(&args, case_sensitive)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(e) = rust_learnins::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

