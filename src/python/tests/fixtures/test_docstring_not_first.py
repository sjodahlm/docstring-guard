
# Fixture: tests that a function with a docstring not as the first statement is flagged
def docstring_not_first():
    print("Hello World")
    """prints 'Hello World'"""
