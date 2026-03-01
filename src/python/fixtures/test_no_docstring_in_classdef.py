
# Fixture: tests that a class and its methods missing docstrings are flagged, except dunder methods
class HelloWorld:
    def __init__(self):
        pass
    def no_docstring(self):
        print("Hello World")