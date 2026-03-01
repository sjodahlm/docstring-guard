use anyhow::Result;
use std::{fs, path::Path};

pub fn load_file_content(path: impl AsRef<Path>) -> Result<String> {
    fs::read_to_string(path.as_ref()).map_err(anyhow::Error::from)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn ignore_validation(line_number: usize, content: &str) -> bool {
    let mut lines = content.lines();
    if let Some(ignore) = lines.nth(line_number - 1) {
        return remove_whitespace(ignore).contains("#docstring-guard=ignore");
    }
    false
}
