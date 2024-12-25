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
    bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri update
    bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri/models update
    bazel run -- @pnpm --dir $(pwd)/sandbox/svelte-hello-world update

update-python:
    bazel run @@//bazel/third_party:python_requirements

# Run all testing
test: build bazel-check
    cargo check
    cargo clippy
    bazel test --test_keep_going //...

# Build everything
build: bazel-update npm-install
    cargo build
    bazel build --keep_going //...

npm-install:
    bazel run -- @pnpm//:pnpm --dir $(pwd) install --lockfile-only # Only this one is needed to update pnpm-lock.yaml
    bazel run -- @pnpm --dir $(pwd) install --recursive  # FIXME: Are the next ones needed?
    bazel run -- @pnpm --dir $(pwd)/sandbox/svelte-hello-world install
    bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri install
    bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri/models install


# Run all the Bazel targets labelled with 'update' tag
bazel-update:
    scripts/bazel_run_targets.sh update

# Run all the Bazel targets labelled with 'check' tag
bazel-check:
    scripts/bazel_run_targets.sh check

# Run all the `oci_load` rules: These rules will generate OCI containers and load them into the local registry
bazel-load-oci:
    scripts/bazel_run_oci_load_targets.sh

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
    docker system prune --volumes --force

# Reset: removes all temporary files and recreates the workspace (Cargo and Bazel). This can take a while
reset: clean build test
