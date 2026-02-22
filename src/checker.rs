use anyhow::{Context, Result};
use rustpython_parser::{Mode, ast::Stmt, parse, text_size::TextRange};
use std::{fs, path::Path};

pub struct MissingDocstring {
    pub file_name: String,
    pub name: String,
    pub line_number: usize,
}

fn file_name_to_string(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());
    file_name
}

fn load_file_content(path: impl AsRef<Path>) -> Result<String> {
    let content = fs::read_to_string(path.as_ref())
        .with_context(|| format!("Faild to read file to string {}", path.as_ref().display()));

    content
}

fn get_line_number(content: &str, range: TextRange) -> usize {
    let start = range.start().to_usize();
    let slice = &content[..start];
    let line_number = slice.chars().filter(|c| *c == '\n').count();

    line_number + 1
}

fn is_docstring(stmt: &Stmt) -> bool {
    stmt.as_expr_stmt()
        .and_then(|e| e.value.as_constant_expr())
        .map_or(false, |c| c.value.is_str())
}

pub fn check_file_for_docstrings(file_path: impl AsRef<Path>) -> Result<Vec<MissingDocstring>> {
    let path = file_path.as_ref();
    let file_name = file_name_to_string(path);
    let content = load_file_content(path)?;
    let module: rustpython_parser::ast::Mod = parse(&content, Mode::Module, &file_name)
        .with_context(|| format!("Faild to parse file content from {}", path.display()))?;

    let mut missing_docstrings: Vec<MissingDocstring> = vec![];

    let module_body = &module.as_module().unwrap().body;
    for stmt in module_body.iter() {
        if stmt.is_function_def_stmt() {
            let func_stmt = &stmt.as_function_def_stmt().unwrap();
            let range = func_stmt.range;
            let id = func_stmt.name.as_str();
            let docstring = &func_stmt.body[0];

            match is_docstring(&docstring) {
                true => continue,
                false => {
                    let entry = MissingDocstring {
                        file_name: file_name.clone(),
                        name: id.to_string(),
                        line_number: get_line_number(&content, range),
                    };
                    missing_docstrings.push(entry);
                }
            };
        }
    }

    Ok(missing_docstrings)
}
