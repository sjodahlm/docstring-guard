# pre-commit-pydocstring

A git pre-commit hook that checks staged Python files for missing docstrings on functions and classes.

## Architecture

- **Rust binary** (`docstring-checker`) performs AST analysis on Python source files

## Status

Work in progress. Currently supports single-file docstring checking via CLI.
Planned: multi-file support via pre-commit framework integration.