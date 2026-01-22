# Plan Mode Rules

This file contains non-obvious architectural constraints and patterns for planning changes.

## Architecture Constraints (Hidden)

- **Bazel-first design**: All build logic must work through Bazel - Cargo/npm are secondary for IDE support only
- **Feature flag dependencies**: Rust features must be explicitly listed in `all_features` dict - Cargo.toml feature deps not auto-detected
- **Generated file coupling**: Schema.rs, protobuf files, and BUILD files are tightly coupled - changes require multi-step update process

## Multi-Language Coordination

- **C++/Rust/TS protobuf**: Same .proto files generate code for all three languages - changes affect multiple apps
- **Shared libraries**: Changes in `libraries/` can impact multiple apps - check reverse dependencies before modifying
- **Test infrastructure**: `with_postgres_test()` wrapper used across C++ and potentially Rust tests - changes affect multiple test suites

## Build System Patterns (Non-Standard)

- **Custom macros required**: Direct use of `rust_library`, `cc_test` discouraged - use project-specific wrappers
- **Update/check workflow**: Generated files must be updated (`just bazel-update`) then verified (`just bazel-check`) before commit
- **Docker in tests**: Postgres tests automatically manage containers - no manual Docker setup needed

## Dependency Management Strategy

- **Three-way sync**: Rust deps in Cargo.toml → crates_vendor → patched BUILD files (all three must stay in sync)
- **npm two-step**: pnpm update → protobuf copy (both steps required for complete update)
- **Bazel module updates**: Separate script (`scripts/update_bazel_modules.sh`) handles MODULE.bazel updates

## Testing Architecture

- **Test data distinction**: `test_compile_data` (migrations, schemas) vs `test_data` (runtime files) affects Bazel caching
- **Integration test constraints**: Must be in `tests/test_*.rs` (flat structure) - nested directories won't work
- **Shared test code**: Use `shared_srcs` in test suites, not separate test utility crates

## Common Architectural Gotchas

- **Debug assertions always on**: Tauri macros require debug assertions even in release builds - can't disable
- **C++23 minimum**: No fallback to older C++ standards - all code must be C++23 compatible
- **Rust edition locked**: Edition 2021 hardcoded in multiple places - upgrading requires coordinated changes
