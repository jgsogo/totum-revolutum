"""Utility function for dedenting multi-line strings in Starlark."""

def dedent(text):
    """Remove common leading whitespace from every line in text.

    This is a simple implementation of Python's textwrap.dedent for Starlark.
    It removes any common leading whitespace from every line in the input text.

    Args:
        text: A multi-line string to dedent

    Returns:
        The dedented text with common leading whitespace removed

    Example:
        ```python
        dedent('''
            Hello
            World
        ''')
        # Returns: "Hello\nWorld\n"
        ```
    """
    lines = text.splitlines()
    if not lines:
        return text

    # Find minimum indentation (ignoring empty lines)
    min_indent = None
    for line in lines:
        stripped = line.lstrip()
        if stripped:
            indent = len(line) - len(stripped)
            if min_indent == None or indent < min_indent:
                min_indent = indent

    if min_indent == None or min_indent == 0:
        return text

    # Remove the common indentation
    result = []
    for line in lines:
        if len(line) >= min_indent:
            result.append(line[min_indent:])
        else:
            result.append(line)

    return "\n".join(result)
