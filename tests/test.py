
# TODO: to be included in actual unit testing of the rust crate

# Checks valid docstring
def valid_docstring():
    """prints 'Hello World'"""
    print("Hello World")

# Checks invalid docstring placement
def docstring_not_first(): #docstring-guard=ignore
    print("Hello World")
    """prints 'Hello World'"""

# Checks no docstring
def no_docstring(): #docstring-guard=ignore
    print("Hello World")

class HelloWorld: #docstring-guard=ignore
    def __init__(self):
        pass
    def __check(self): #docstring-guard=ignore
        pass