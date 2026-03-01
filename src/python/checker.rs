use anyhow::{Context, Result};
use colored::Colorize;
use rustpython_parser::{
    Mode,
    ast::{Identifier, Stmt, StmtClassDef, StmtFunctionDef},
    parse,
    text_size::TextRange,
};
use std::path::Path;

use crate::{
    MissingDocstring,
    utils::{ignore_validation, load_file_content},
};

trait Documentable {
    fn body(&self) -> &[Stmt];
    fn name(&self) -> &Identifier;
    fn range(&self) -> TextRange;
}

impl Documentable for StmtFunctionDef {
    fn body(&self) -> &[Stmt] {
        &self.body
    }
    fn name(&self) -> &Identifier {
        &self.name
    }
    fn range(&self) -> TextRange {
        self.range
    }
}

impl Documentable for StmtClassDef {
    fn body(&self) -> &[Stmt] {
        &self.body
    }
    fn name(&self) -> &Identifier {
        &self.name
    }
    fn range(&self) -> TextRange {
        self.range
    }
}

fn is_docstring(stmt: &Stmt) -> bool {
    stmt.as_expr_stmt()
        .and_then(|e| e.value.as_constant_expr())
        .map_or(false, |c| c.value.is_str())
}

fn is_dunder(name: &str) -> bool {
    if name.starts_with("__") && name.ends_with("__") {
        return true;
    }
    false
}

fn get_line_number(content: &str, range: TextRange) -> usize {
    let start = range.start().to_usize();
    let slice = &content[..start];
    slice.chars().filter(|c| *c == '\n').count() + 1
}

fn check_body_for_docstrings(path: &Path, content: &str, stmts: &[Stmt]) -> Vec<MissingDocstring> {
    let mut missing_docstrings: Vec<MissingDocstring> = vec![];

    for stmt in stmts.iter() {
        let entry: Option<MissingDocstring> = if let Some(s) = stmt.as_function_def_stmt() {
            check_body_for_docstring(path, s, content)
        } else if let Some(s) = stmt.as_class_def_stmt() {
            if let Some(missing) = check_body_for_docstring(path, s, content) {
                missing_docstrings.push(missing);
            }
            missing_docstrings.extend(check_body_for_docstrings(path, content, s.body()));
            continue;
        } else {
            None
        };

        if let Some(e) = entry {
            missing_docstrings.push(e);
        }
    }
    missing_docstrings
}

fn check_body_for_docstring(
    path: &Path,
    stmt: &impl Documentable,
    content: &str,
) -> Option<MissingDocstring> {
    let range = stmt.range();
    let id = stmt.name().as_str();
    let line_number = get_line_number(content, range);

    if let Some(docstring) = stmt.body().first()
        && !is_dunder(id)
        && !ignore_validation(line_number, content)
    {
        if !is_docstring(docstring) {
            let entry = MissingDocstring {
                file_name: path.display().to_string(),
                name: id.to_string(),
                line_number,
            };
            return Some(entry);
        }
    }
    None
}

pub fn check_file_for_docstrings(file_path: impl AsRef<Path>) -> Result<Vec<MissingDocstring>> {
    let path = file_path.as_ref();
    let content = load_file_content(path).with_context(|| {
        format!(
            "{} {} failed to read",
            "error:".red(),
            format!("{}:", path.display()).bold(),
        )
    })?;
    let module: rustpython_parser::ast::Mod = parse(&content, Mode::Module, "<unknown>")
        .with_context(|| {
            format!(
                "{} {} failed to parse",
                "error:".red(),
                format!("{}:", path.display()).bold(),
            )
        })?;

    let mut missing_docstrings: Vec<MissingDocstring> = vec![];
    if let Some(module) = &module.as_module() {
        missing_docstrings.extend(check_body_for_docstrings(path, &content, &module.body));
    }
    Ok(missing_docstrings)
}
