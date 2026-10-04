# Advance Mode Rules

This file contains non-obvious patterns for advance mode (with MCP and Browser access).

## Bazel Custom Macros (Must Use)

- **Never use `rust_library` directly**: Always use `rust_library_tests_and_docs()` from `//bazel/rust:defs.bzl` - it creates feature-specific targets and test suites automatically
- **Diesel schema updates**: After modifying migrations, run `bazel run //<package>:<schema_name>.update` to regenerate schema.rs files (not `diesel migration run`)
- **Integration test location**: Rust integration tests MUST be in `tests/test_*.rs` (not `tests/**/*.rs`) - the glob pattern in `rust_test_suite` only matches top-level files

## Build Patterns (Non-Standard)

- **Rust debug assertions**: Always on in Bazel builds (`-Cdebug-assertions=on`) because Tauri macros have different signatures with/without them - don't try to disable
- **C++ external deps warnings**: Suppressed via `--per_file_copt=external/.*@-w,-Wno-all` - don't add `-Werror` to external code
- **Generated files**: Schema.rs files are auto-patched to skip rustfmt - don't manually format them

## Testing Patterns (Critical)

- **Postgres tests**: Wrap test targets with `with_postgres_test()` instead of using `cc_test` or `rust_test` directly - it manages Docker container lifecycle
- **Test data**: Use `test_compile_data` for files needed during compilation (like migrations), `test_data` for runtime files
- **Shared test helpers**: Use `shared_srcs` parameter in `rust_test_suite` for common test utilities (not separate crate)

## Dependency Management (Non-Obvious)

- **Rust deps update**: Run `just update-rust` (not `cargo update` alone) - it also runs `crates_vendor` and applies patches to generated BUILD files
- **npm deps update**: Run `just update-npm` - it updates pnpm AND copies generated protobuf TS files to workspace (two-step process)
- **Feature dependencies**: Must explicitly list feature dependencies in `all_features` dict for `rust_library_tests_and_docs()` - Cargo.toml feature deps not auto-detected by Bazel

## File Generation Workflow

1. Update source (proto, migration, etc.)
2. Run `just bazel-update` to regenerate all files tagged with `update`
3. Run `just bazel-check` to verify diffs (fails if workspace files don't match generated)
4. Commit both source and generated files

## Common Gotchas

- **node_modules in workspace**: Created by `just npm-install` for IDE support only - Bazel uses its own in build directory
- **C++23 required**: All C++ code must compile with `-std=c++23` - older standards not supported
- **Rust edition 2021**: Hardcoded in MODULE.bazel and rustfmt.toml - don't use 2024 edition yet
