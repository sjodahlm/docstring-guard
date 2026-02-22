
# TODO: to be included in actual unit testing of the rust crate

# Checks valid docstring
def valid_docstring():
    """prints 'Hello World'"""
    print("Hello World")

# Checks invalid docstring placement
def docstring_not_first():
    print("Hello World")
    """prints 'Hello World'"""

# Checks no docstring
def no_docstring():
    print("Hello World")