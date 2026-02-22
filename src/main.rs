use std::env;
use std::process::exit;

use crate::checker::check_file_for_docstrings;
mod checker;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("no file provided");
        exit(0);
    }

    for argv in args {
        match check_file_for_docstrings(argv) {
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
}
