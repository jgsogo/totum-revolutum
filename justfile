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
    bazel run -- @pnpm --dir $(pwd)/sandbox/svelte-hello-world update

# Run all testing
test:
    bazel test //...
    cargo check

# Run all the Bazel targets labelled with 'update' tag
bazel-update:
    scripts/bazel_run_update_targets.sh
