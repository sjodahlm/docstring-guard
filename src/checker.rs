use colored::Colorize;

use anyhow::{Context, Result};
use rustpython_parser::{Mode, ast::Stmt, parse, text_size::TextRange};
use std::{fs, path::Path};

pub struct MissingDocstring {
    pub file_name: String,
    pub name: String,
    pub line_number: usize,
}

fn load_file_content(path: impl AsRef<Path>) -> Result<String> {
    fs::read_to_string(path.as_ref()).with_context(|| {
        format!(
            "{} {} failed to read",
            format!("{}:", path.as_ref().display()).bold(),
            "error:".red()
        )
    })
}

fn get_line_number(content: &str, range: TextRange) -> usize {
    let start = range.start().to_usize();
    let slice = &content[..start];
    slice.chars().filter(|c| *c == '\n').count() + 1
}

fn is_docstring(stmt: &Stmt) -> bool {
    stmt.as_expr_stmt()
        .and_then(|e| e.value.as_constant_expr())
        .map_or(false, |c| c.value.is_str())
}

pub fn check_file_for_docstrings(file_path: impl AsRef<Path>) -> Result<Vec<MissingDocstring>> {
    let path = file_path.as_ref();
    let content = load_file_content(path)?;
    let module: rustpython_parser::ast::Mod = parse(&content, Mode::Module, "<unknown>")
        .with_context(|| {
            format!(
                "{} {} failed to parse",
                format!("{}:", path.display()).bold(),
                "error:".red()
            )
        })?;

    let mut missing_docstrings: Vec<MissingDocstring> = vec![];
    if let Some(module) = &module.as_module() {
        for stmt in module.body.iter() {
            if let Some(func_stmt) = &stmt.as_function_def_stmt() {
                let range = func_stmt.range;
                let id = func_stmt.name.as_str();

                if let Some(docstring) = &func_stmt.body.first() {
                    if !is_docstring(docstring) {
                        let entry = MissingDocstring {
                            file_name: path.display().to_string(),
                            name: id.to_string(),
                            line_number: get_line_number(&content, range),
                        };
                        missing_docstrings.push(entry);
                    }
                }
            }
        }
    }
    Ok(missing_docstrings)
}
