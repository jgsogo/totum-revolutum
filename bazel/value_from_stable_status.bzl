"""A rule to extract a value from the 'stable-status.txt' file"""

def value_from_stable_status(name, key, default, **kwargs):
    """Creates a target that contains the value of the given key in the 'stable-status.txt' file

    Args:
        name(str): the generated target name
        key(str): the key to extract from the 'stable-status.txt' file.
        default(str): default value if the given key is not found
        **kwargs: extra arguments for the genrule.
    """
    native.genrule(
        name = name,
        outs = ["{}.txt".format(name)],
        cmd_bash = """
            h=$$(grep ^{key} bazel-out/stable-status.txt | cut -d' ' -f2 || echo "{default}");
            echo "$$h" > $@
        """.format(key = key, default = default),
        stamp = 1,  # So this rule will be invalidated when stamping
        **kwargs
    )
