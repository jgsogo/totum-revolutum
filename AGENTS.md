# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Build System - Bazel (Non-Standard Patterns)

- **Custom Rust macro**: Use `rust_library_tests_and_docs()` from `//bazel/rust:defs.bzl` instead of standard `rust_library`. It creates multiple targets: `name/vanilla` (no features), `name/<feature>` (per feature), `name` (all features), plus `tests`, `integration_tests`, `doc`, and `doc/tests`.
- **Diesel schema generation**: Use `diesel_print_schema()` from `//bazel/rust/diesel:diesel_print_schema.bzl`. It auto-generates `.update` and `.test` targets. Run `bazel run //<package>:<name>.update` to update schema files.
- **Postgres tests**: Use `with_postgres_test()` from `//bazel/containers/postgres:with_postgres_run.bzl` for tests requiring Postgres. It spins up a Docker container and provides env vars (POSTGRES_DB, POSTGRES_USER, etc.).
- **Update targets**: Run `just bazel-update` to execute all Bazel targets tagged with `update` (schema generation, protobuf copies, etc.).
- **Check targets**: Run `just bazel-check` to execute all Bazel targets tagged with `check` (diff tests for generated files).

## Monorepo Structure

- **Multi-language**: C++ (C++23), Rust (2021 edition), TypeScript, Python (3.14)
- **Package managers**: Bazel (primary), Cargo (Rust workspace), pnpm (Node workspaces), Just (task runner)
- **Apps**: `apps/board_games/` (C++ engine + TS webapp), `apps/finances/` (Tauri + Django), `apps/photodb/`, `apps/syncronia/`
- **Libraries**: Shared code in `libraries/` (Rust, C++, TS)

## Commands (Non-Standard)

- **Update all deps**: `just update` (updates Rust, npm, Python, Bazel modules, pre-commit)
- **Update Rust deps**: `just update-rust` (runs `cargo update`, then `bazel run //bazel/third_party:crates_vendor`, then patches with `//bazel/third_party:patch_crates_vendor`)
- **Update npm deps**: `just update-npm` (runs pnpm update, then copies generated protobuf TS files to workspace)
- **Full test**: `just full-test` (build + bazel-check + test + format)
- **C++ compile commands**: `just cpp-compile-commands` (generates compile_commands.json for Clangd)
- **npm install**: `just npm-install` (creates node_modules in workspace for IDE, but Bazel uses its own in build dir)

## Code Style (Project-Specific)

- **Rust**: Max width 120, edition 2021, `use_field_init_shorthand = true`, `reorder_imports = true`
- **C++**: C++23 standard, `-Werror` enabled, `-Wno-deprecated-declarations` for protobuf <33.1
- **Bazel C++**: Use `--per_file_copt=external/.*@-w,-Wno-all` to suppress warnings in external deps
- **Rust debug assertions**: Always enabled in Bazel builds (`-Cdebug-assertions=on`) because Tauri macros require them
- **Generated schema.rs**: Auto-patched to skip rustfmt (see `//bazel/rust/diesel:skip_rustfmt.patch`)

## Testing (Non-Obvious)

- **Rust integration tests**: Must be in `tests/test_*.rs` (not `tests/**/*.rs`), use `rust_test_suite` with `shared_srcs` for common helpers
- **C++ tests**: Use Catch2, test files named `test_*.cpp`, often wrapped in `with_postgres_test()` for DB tests
- **TypeScript tests**: Vitest, test files named `*.spec.ts`
- **Test data**: Use `test_compile_data` for files needed at compile time, `test_data` for runtime files

## Protobuf

- **Auto-copy to workspace**: After updating protos, run targets like `//apps/board_games/engine/protocol:engine_ts_proto.copy` to copy generated TS files to workspace
- **Update all protos**: `just update-npm` handles all protobuf TS copies

## Docker/Containers

- **Postgres tests**: Automatically managed by `with_postgres_test()`, uses image `postgres:17`
- **Container name pattern**: `<package_path>_<target_name>` (slashes replaced with underscores)

## Workspace Status

- **Stamping**: Use `--config=stamp` to enable workspace status stamping (runs `bazel/workspace_status.sh`)
- **CI config**: Use `--config=ci` for CI builds (verbose failures, show progress, test output=all)
- **Release config**: Use `--config=release` for optimized builds (LTO, opt-level=3, no debug assertions)
