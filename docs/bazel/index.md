# Bazel Rules Overview

This section contains documentation for all custom Bazel rules and macros in the Totum Revolutum monorepo.

## Categories

### Core Utilities

Essential utilities that provide common functionality:

- **[dedent](dedent.md)** - Remove common leading whitespace from multi-line strings
- **[run_copy_to_workspace](run_copy_to_workspace.md)** - Copy generated files back to the workspace for version control
- **[sh_with_runfiles_binary](sh_with_runfiles_binary.md)** - Create shell scripts that can access Bazel runfiles

### Rust Rules

Rules and macros for Rust development:

- **[rust_library_tests_and_docs](rust/defs.md)** - Comprehensive macro that creates:
  - Multiple library targets with different feature combinations
  - Unit tests and integration tests
  - Documentation targets
  - Doc tests

- **[rust_docs](rust/docs/defs.md)** - Generate combined documentation for multiple Rust crates in a single output

### Diesel ORM Rules

Rules for working with Diesel, the Rust ORM:

- **[diesel_print_schema](rust/diesel/diesel_print_schema.md)** - Generate `schema.rs` files from database schema
- **[diesel_setup](rust/diesel/diesel_setup.md)** - Setup and configuration utilities for Diesel

### Container Rules

Rules for managing Docker containers during builds and tests:

- **[with_postgres_run](containers/postgres/with_postgres_run.md)** - Run binaries with a PostgreSQL container
- **[with_postgres_test](containers/postgres/with_postgres_run.md#with_postgres_test)** - Run tests with a PostgreSQL container

## Design Patterns

### Feature-Based Rust Libraries

The `rust_library_tests_and_docs` macro implements a pattern where a single Rust crate is built multiple times with different feature combinations:

```python
rust_library_tests_and_docs(
    name = "mylib",
    srcs = ["src/lib.rs"],
    all_features = {
        "feature1": ["feature1"],
        "feature2": ["feature2"],
        "all": ["feature1", "feature2"],
    },
)
```

This creates:
- `mylib/vanilla` - No features
- `mylib/feature1` - Only feature1
- `mylib/feature2` - Only feature2
- `mylib/all` - All features
- `mylib` - All features (default)

### Database Schema Generation

The Diesel rules implement a pattern for keeping generated schema files in sync:

1. Generate schema from database: `diesel_print_schema()`
2. Create `.update` target to copy to workspace
3. Create `.test` target to verify it's up-to-date
4. Run `bazel run //:schema.update` to update
5. Run `bazel test //:schema.test` to verify

### Container-Based Testing

The PostgreSQL rules provide a clean way to run tests that need a database:

```python
with_postgres_test(
    name = "my_test",
    binaries = [":my_test_binary"],
    env = {
        "POSTGRES_DB": "testdb",
    },
)
```

The container is automatically started before the test and stopped after.

## Best Practices

1. **Always use `rust_library_tests_and_docs`** instead of plain `rust_library` for better testing and documentation
2. **Keep generated files in version control** using the `.update` targets
3. **Use diff tests** to ensure generated files stay in sync
4. **Document your rules** with docstrings for automatic documentation generation
5. **Use the `dedent` utility** for multi-line docstrings in Bazel rules

## Adding New Rules

See the [Contributing Guide](docs/README.md) for instructions on adding documentation for new Bazel rules.
