# pre-commit-pydocstring

A git pre-commit hook that checks staged Python files for missing docstrings on functions and classes.

## Architecture

- **Shell wrapper** (`hook`) handles git interaction and orchestration
- **Rust binary** (`docstring-checker`) performs AST analysis on Python source files

## Status

Work in progress.
