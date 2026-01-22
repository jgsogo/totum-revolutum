# Totum Revolutum - Bazel Rules Documentation

Welcome to the documentation for custom Bazel rules and macros used in the Totum Revolutum monorepo.

## Overview

This documentation covers all custom Bazel rules, macros, and utilities that extend Bazel's functionality for this project. The rules are organized by category:

### 📦 Core Utilities

Essential utilities used across the monorepo:

- **[dedent](bazel/dedent.md)** - String dedentation utility for multi-line docstrings
- **[run_copy_to_workspace](bazel/run_copy_to_workspace.md)** - Copy generated files back to the workspace
- **[sh_with_runfiles_binary](bazel/sh_with_runfiles_binary.md)** - Create shell scripts with runfiles support

### 🦀 Rust Rules

Custom rules for Rust development:

- **[rust_library_tests_and_docs](bazel/rust/defs.md)** - Create Rust libraries with multiple feature configurations, tests, and documentation
- **[rust_docs](bazel/rust/docs/defs.md)** - Generate combined documentation for multiple Rust crates

#### Diesel ORM

Rules for working with Diesel (Rust ORM):

- **[diesel_print_schema](bazel/rust/diesel/diesel_print_schema.md)** - Generate Diesel schema files from database
- **[diesel_setup](bazel/rust/diesel/diesel_setup.md)** - Setup utilities for Diesel

### 🐘 Container Rules

Rules for managing Docker containers in tests:

- **[with_postgres_run](bazel/containers/postgres/with_postgres_run.md)** - Run binaries with PostgreSQL container
- **[with_postgres_test](bazel/containers/postgres/with_postgres_run.md#with_postgres_test)** - Run tests with PostgreSQL container

## Quick Start

### Generating Documentation

To regenerate this documentation from the Bazel rules:

```bash
bazel build //bazel:all_docs
```

The generated Markdown files will be in `bazel-bin/bazel/`.

### Using the Rules

All rules are available by loading them from their respective `.bzl` files:

```python
# Rust utilities
load("//bazel/rust:defs.bzl", "rust_library_tests_and_docs")
load("//bazel/rust/docs:defs.bzl", "rust_docs")

# Diesel utilities
load("//bazel/rust/diesel:diesel_print_schema.bzl", "diesel_print_schema")

# Container utilities
load("//bazel/containers/postgres:with_postgres_run.bzl", "with_postgres_test")

# Core utilities
load("//bazel:dedent.bzl", "dedent")
```

## Contributing

To add documentation for new Bazel rules, see the [Contributing Guide](bazel/docs/README.md).

## Project Structure

```
bazel/
├── BUILD.bazel              # Main build file with all_docs target
├── dedent.bzl              # String utilities
├── run_copy_to_workspace.bzl
├── sh_with_runfiles_binary.bzl
├── rust/
│   ├── defs.bzl            # Rust library macros
│   ├── docs/
│   │   └── defs.bzl        # Rust documentation rules
│   └── diesel/
│       ├── diesel_print_schema.bzl
│       └── diesel_setup.bzl
└── containers/
    └── postgres/
        └── with_postgres_run.bzl
```

## Technology Stack

- **Bazel** - Build system
- **Stardoc** - Documentation generator for Bazel rules
- **MkDocs Material** - Documentation site generator
- **Rust** - Primary programming language for many rules
- **Python** - Django and other utilities
- **C++** - Game engines and performance-critical code

## License

See [LICENSE](https://github.com/yourusername/totum-revolutum/blob/main/LICENSE) for details.
