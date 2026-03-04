use anyhow::Error;
use colored::Colorize;
use docstring_guard::{check_file_for_docstrings, MissingDocstring};
use std::process::exit;
use std::{collections::HashSet, env};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut docstring_fails: Vec<MissingDocstring> = vec![];
    let mut errors: Vec<Error> = vec![];
    let mut fails: HashSet<String> = HashSet::new();

    for argv in &args {
        match check_file_for_docstrings(argv) {
            Ok(missing_docstrings) => {
                docstring_fails.extend(missing_docstrings);
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }

    for err in errors.iter() {
        eprintln!("{} - {}", err, err.root_cause());
    }

    for missing_docstring in docstring_fails.iter() {
        println!(
            "{} {} no docstring in '{}'",
            format!(
                "{}:{}:",
                missing_docstring.file_name, missing_docstring.line_number
            )
            .bold(),
            "failed:".red(),
            missing_docstring.name,
        );
        fails.insert(missing_docstring.file_name.clone());
    }

    if !errors.is_empty() || !docstring_fails.is_empty() {
        let passed: usize = args.len() - fails.len() - errors.len();
        println!(
            "{} {} violations found in {} {} ({} errors, {} passed)",
            "docstring-guard:".bold(),
            docstring_fails.len(),
            fails.len(),
            if fails.len() == 1 { "file" } else { "files" },
            errors.len(),
            passed,
        );
        exit(1)
    }

    exit(0)
}
