_default: _just-check
    @{{ just_executable() }} --choose  # Requires 'fzf' (https://formulae.brew.sh/formula/fzf)

_just-check:
    {{ just_executable() }} --unstable --fmt --check

# Updates all the dependencies (MODULE.bazel and 3rd parties not included)
update:
    pre-commit autoupdate
    cargo update
    bazel run -- @pnpm --dir $(pwd) update --recursive --workspace  # FIXME: This command should include per-project ones in the following lines
    bazel run -- @pnpm --dir $(pwd)/sandbox/tauri-hello-world update
    bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri update
    bazel run -- @pnpm --dir $(pwd)/sandbox/svelte-hello-world update
    bazel run @@//bazel/third_party:python_requirements

# Run all testing
test:
    cargo check
    cargo clippy
    bazel test //...

# Build everything
build:
    cargo build
    bazel build //...

# Run all the Bazel targets labelled with 'update' tag
bazel-update:
    scripts/bazel_run_targets.sh update

# Execute tokei: prints statistics about the repository
tokei:
    tokei --sort lines --compact

# Shows the documentation
doc:
    cargo doc --open --document-private-items --all-features --workspace

# Removes temporary files (free disk space)
clean:
    cargo clean
    bazel clean

# Reset: removes all temporary files and recreates the workspace (Cargo and Bazel). This can take a while
reset: clean build test
