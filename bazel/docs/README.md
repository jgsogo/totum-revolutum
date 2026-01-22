# Bazel Rules Documentation

This directory contains automatically generated documentation for all custom Bazel rules in this repository using [Stardoc](https://github.com/bazelbuild/stardoc).

## Generating Documentation

To generate documentation for all Bazel rules, run:

```bash
bazel build //bazel:all_docs
```

The generated markdown files will be available in:
- `bazel-bin/bazel/*.md` - Core utility rules
- `bazel-bin/bazel/rust/*.md` - Rust-related rules
- `bazel-bin/bazel/rust/diesel/*.md` - Diesel ORM rules
- `bazel-bin/bazel/rust/docs/*.md` - Rust documentation generation rules
- `bazel-bin/bazel/containers/postgres/*.md` - PostgreSQL container rules

## Available Documentation

### Core Utilities
- **dedent.md** - String dedentation utility
- **run_copy_to_workspace.md** - Copy files to workspace utility
- **sh_with_runfiles_binary.md** - Shell script with runfiles support

### Rust Rules
- **rust/defs.md** - `rust_library_tests_and_docs` macro for creating Rust libraries with multiple feature configurations
- **rust/docs/defs.md** - `rust_docs` rule for generating combined documentation for multiple Rust crates
- **rust/diesel/diesel_print_schema.md** - `diesel_print_schema` macro for generating Diesel schema files
- **rust/diesel/diesel_setup.md** - Diesel setup utilities

### Container Rules
- **containers/postgres/with_postgres_run.md** - `with_postgres_run` and `with_postgres_test` rules for running tests with PostgreSQL

## Limitations

Some rules cannot be documented with Stardoc due to missing `bzl_library` targets in their dependencies:

- **Django rules** (`bazel/python/django/project/*.bzl`) - Depend on `@aspect_rules_py//py:defs.bzl` which doesn't provide bzl_library targets

## Adding Documentation for New Rules

To add documentation for a new Bazel rule:

1. **Create a `bzl_library` target** in the BUILD.bazel file where your .bzl file is located:

```python
load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

bzl_library(
    name = "my_rule",
    srcs = ["my_rule.bzl"],
    visibility = ["//visibility:public"],
    deps = [
        # List dependencies here
    ],
)
```

2. **Create a `stardoc` target** to generate the documentation:

```python
load("@stardoc//stardoc:stardoc.bzl", "stardoc")

stardoc(
    name = "my_rule_doc",
    input = "my_rule.bzl",
    out = "my_rule.md",
    visibility = ["//visibility:public"],
    deps = [":my_rule"],
)
```

3. **Add the documentation target** to `//bazel:all_docs` filegroup in `bazel/BUILD.bazel`:

```python
filegroup(
    name = "all_docs",
    srcs = [
        # ... existing docs ...
        "//path/to/your:my_rule_doc",
    ],
    visibility = ["//visibility:public"],
)
```

4. **Build and verify**:

```bash
bazel build //bazel:all_docs
cat bazel-bin/path/to/your/my_rule.md
```

## Documentation Best Practices

When writing Bazel rules, follow these practices for better documentation:

1. **Add docstrings to rules and macros**:
```python
def my_macro(name, srcs, **kwargs):
    """Creates a custom target with special handling.

    Args:
        name: The name of the target
        srcs: Source files to process
        **kwargs: Additional arguments passed to underlying rule
    """
```

2. **Document rule attributes**:
```python
my_rule = rule(
    implementation = _my_rule_impl,
    attrs = {
        "srcs": attr.label_list(
            doc = "Source files to process",
            allow_files = True,
        ),
    },
    doc = "This rule does something useful",
)
```

3. **Use the `dedent` utility** for multi-line docstrings:
```python
load("//bazel:dedent.bzl", "dedent")

my_rule = rule(
    doc = dedent("""\
        This is a long description
        that spans multiple lines
        and maintains proper indentation.
    """),
    # ...
)
```

## See Also

- [Stardoc Documentation](https://github.com/bazelbuild/stardoc)
- [Bazel Documentation Guide](https://bazel.build/rules/bzl-style#documentation)
