# docstring-guard

A git pre-commit hook that checks Python files for missing docstrings on functions and classes, built with Rust for fast AST-based analysis.

## Requirements

- Rust and Cargo (installed via [rustup](https://rust-lang.org/tools/install/))

- [pre-commit](https://pre-commit.com)

After installation, verify:

```bash
rustc --version
cargo --version
pre-commit --version
```

## Installation

Add the following to your `.pre-commit-config.yaml`:
```yaml
repos:
  - repo: https://github.com/sjodahlm/docstring-guard
    rev: v0.1.0
    hooks:
      - id: docstring-guard
```

Then install the hook:
```bash
pre-commit install
```

## Usage

The hook runs automatically on `git commit` and checks all staged Python files for missing docstrings.

### Ignoring a function or class

Add `#docstring-guard=ignore` as an inline comment on the definition line:
```python
def internal_helper(): #docstring-guard=ignore
    pass
```

### Dunder methods

Dunder methods such as `__init__` and `__str__` are automatically skipped.

### Example Output

```
$ git commit -m "feat: add new feature"
check missing docstrings.................................................Failed
- hook id: docstring-guard
- exit code: 1

src/main.py:10: failed: no docstring in 'my_function'
src/utils.py:5: failed: no docstring in 'MyClass'
docstring-guard: 2 violations found in 2 files (0 errors, 0 passed)
```

## Limitations

- Currently only supports Python. Multi-language support is planned.

## License

MIT