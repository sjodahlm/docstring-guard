use std::process::exit;

use crate::checker::check_file_for_docstrings;
mod checker;

fn main() {
    let path: &str = "/home/sjodahlm/Documents/projects/pre-commit-pydocstring/tests/test.py";
    match check_file_for_docstrings(path) {
        Ok(missing_docstrings) => {
            for missing_docstring in missing_docstrings.iter() {
                println!(
                    "#{}, {}, {}",
                    missing_docstring.file_name,
                    missing_docstring.name,
                    missing_docstring.line_number
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
            eprintln!("Caused by: {}", err.root_cause());
            exit(1);
        }
    }
}
