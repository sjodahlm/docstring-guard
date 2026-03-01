
# Fixture: tests that functions with #docstring-guard=ignore are not flagged
def ignore_this_function(): #docstring-guard=ignore
    print("Hello World")