use std::env;
use std::process::exit;

use anyhow::Error;

use crate::checker::{MissingDocstring, check_file_for_docstrings};
mod checker;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("no *.py file provided");
        println!("skipping docstring check...");
        exit(0);
    }

    let mut docstring_errors: Vec<MissingDocstring> = vec![];
    let mut errors: Vec<Error> = vec![];
    for argv in args {
        match check_file_for_docstrings(argv) {
            Ok(missing_docstrings) => {
                docstring_errors.extend(missing_docstrings);
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }

    for err in errors.iter() {
        eprintln!("Error: {err}");
        eprintln!("Caused by: {}", err.root_cause());
    }

    for missing_docstring in docstring_errors.iter() {
        println!(
            "no docstring in function: '{}' in file: '{}' on line: {}",
            missing_docstring.name, missing_docstring.file_name, missing_docstring.line_number
        );
    }

    if !errors.is_empty() || !docstring_errors.is_empty() {
        exit(1)
    }

    exit(0)
}
