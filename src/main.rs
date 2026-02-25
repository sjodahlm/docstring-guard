use anyhow::Error;
use colored::Colorize;
use std::env;
use std::process::exit;

use crate::checker::{MissingDocstring, check_file_for_docstrings};
mod checker;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut docstring_errors: Vec<MissingDocstring> = vec![];
    let mut errors: Vec<Error> = vec![];

    for argv in &args {
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
        eprintln!("{} - {}", err, err.root_cause());
    }

    for missing_docstring in docstring_errors.iter() {
        println!(
            "{} {} no docstring in function '{}'",
            format!(
                "{}:{}:",
                missing_docstring.file_name, missing_docstring.line_number
            )
            .bold(),
            "failed:".red(),
            missing_docstring.name,
        );
    }

    if !errors.is_empty() || !docstring_errors.is_empty() {
        println!();
        // TODO: fix failed files count and passed files in summary
        println!(
            "{} {{TODO}} fails, {} errors ({{TODO}} passed)",
            "docstring-guard:".bold(),
            errors.len()
        );
        exit(1)
    }

    exit(0)
}
