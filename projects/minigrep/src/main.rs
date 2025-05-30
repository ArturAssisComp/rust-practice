use minigrep::{run, ParsedMainArgs};
use std::{env, process};

fn main() {
    let parsed_main_args = ParsedMainArgs::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(&parsed_main_args) {
        eprintln!(
            "{} (specified file: {})",
            e.to_string(),
            parsed_main_args.file_path()
        );
        process::exit(1);
    }
}
