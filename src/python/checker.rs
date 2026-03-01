use crate::{
    MissingDocstring,
    utils::{ignore_validation, load_file_content},
};
use anyhow::{Context, Result};
use colored::Colorize;
use rustpython_parser::{
    Mode,
    ast::{Identifier, Stmt, StmtClassDef, StmtFunctionDef},
    parse,
    text_size::TextRange,
};
use std::path::Path;

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

fn check_statements_for_docstrings(
    path: &Path,
    content: &str,
    stmts: &[Stmt],
) -> Vec<MissingDocstring> {
    let mut missing_docstrings: Vec<MissingDocstring> = vec![];

    for stmt in stmts.iter() {
        let entry: Option<MissingDocstring> = if let Some(s) = stmt.as_function_def_stmt() {
            check_documentable_for_docstring(path, s, content)
        } else if let Some(s) = stmt.as_class_def_stmt() {
            if let Some(missing) = check_documentable_for_docstring(path, s, content) {
                missing_docstrings.push(missing);
            }
            missing_docstrings.extend(check_statements_for_docstrings(path, content, s.body()));
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

fn check_documentable_for_docstring(
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
        missing_docstrings.extend(check_statements_for_docstrings(
            path,
            &content,
            &module.body,
        ));
    }
    Ok(missing_docstrings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("src/python/fixtures/test_valid_docstring.py", vec![])]
    #[case("src/python/fixtures/test_docstring_guard_ignore.py", vec![])]
    #[case("src/python/fixtures/test_docstring_not_first.py",
        vec![
            MissingDocstring {
                file_name:"src/python/fixtures/test_docstring_not_first.py".to_string(),
                name:"docstring_not_first".to_string(),
                line_number:3,
            }
        ]
    )]
    #[case("src/python/fixtures/test_no_docstring_in_classdef.py",
        vec![
            MissingDocstring {
                    file_name:"src/python/fixtures/test_no_docstring_in_classdef.py".to_string(),
                    name:"HelloWorld".to_string(),
                    line_number:3,
            },
            MissingDocstring {
                    file_name:"src/python/fixtures/test_no_docstring_in_classdef.py".to_string(),
                    name:"no_docstring".to_string(),
                    line_number:6,
            }
        ]
    )]
    #[case("src/python/fixtures/test_no_docstring_in_funcdef.py",
        vec![
            MissingDocstring {
                file_name:"src/python/fixtures/test_no_docstring_in_funcdef.py".to_string(),
                name:"no_docstring".to_string(),
                line_number:3,
            }
        ]
    )]
    fn test_check_file_for_docstrings(
        #[case] input: impl AsRef<Path>,
        #[case] expected: Vec<MissingDocstring>,
    ) {
        assert_eq!(expected, check_file_for_docstrings(input).unwrap());
    }

    #[rstest]
    #[case("src/python/fixtures/test_fail_to_read.py")]
    fn test_check_file_for_docstrings_read_errors(#[case] input: impl AsRef<Path>) {
        assert!(check_file_for_docstrings(input).is_err())
    }

    #[rstest]
    #[case("__init__", true)]
    #[case("test", false)]
    #[case("_test", false)]
    fn test_is_dunder(#[case] input: String, #[case] expected: bool) {
        assert_eq!(expected, is_dunder(&input))
    }

    #[rstest]
    #[case(
        "def hello_world():\n\t\"\"\"prints 'Hello World'\"\"\"\n\tprint(\"Hello World\")",
        true
    )]
    #[case("def hello_world():\n\tprint(\"Hello World\")", false)]
    fn test_is_docstring_true(#[case] input: String, #[case] expected: bool) {
        let ast = parse(&input, Mode::Module, "unknown").unwrap();
        let stmt = ast.as_module().unwrap().body[0]
            .as_function_def_stmt()
            .unwrap();
        assert_eq!(expected, is_docstring(&stmt.body()[0]));
    }

    #[rstest]
    #[case(
        "def hello_world(): #docstring-guard=ignore\n\t\n\tprint(\"Hello World\")",
        1,
        true
    )]
    #[case("def hello_world():\n\t\n\tprint(\"Hello World\")", 1, false)]
    fn test_ignore_validation(
        #[case] input: String,
        #[case] line_number: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, ignore_validation(line_number, &input));
    }
}
