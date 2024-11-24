_default: _just-check
    @{{ just_executable() }} --choose  # Requires 'fzf' (https://formulae.brew.sh/formula/fzf)

_just-check:
    {{ just_executable() }} --unstable --fmt --check

# Updates all the dependencies (MODULE.bazel and 3rd parties not included)
update: update-deps update-precommit

update-precommit:
    pre-commit autoupdate

# Updates only the dependencies
update-deps: update-cargo update-npm update-python

update-cargo:
    cargo update

update-npm:
    bazel run -- @pnpm --dir $(pwd) update --recursive --workspace  # FIXME: This command should include per-project ones in the following lines
    bazel run -- @pnpm --dir $(pwd)/sandbox/tauri-hello-world update
    bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri update
    bazel run -- @pnpm --dir $(pwd)/sandbox/svelte-hello-world update

update-python:
    bazel run @@//bazel/third_party:python_requirements

# Run all testing
test: build bazel-check
    cargo check
    cargo clippy
    bazel test //...

# Build everything
build: bazel-update
    cargo build
    bazel build //...

# Run all the Bazel targets labelled with 'update' tag
bazel-update:
    scripts/bazel_run_targets.sh update

# Run all the Bazel targets labelled with 'check' tag
bazel-check:
    scripts/bazel_run_targets.sh check

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
    docker system prune --force


# Reset: removes all temporary files and recreates the workspace (Cargo and Bazel). This can take a while
reset: clean build test
