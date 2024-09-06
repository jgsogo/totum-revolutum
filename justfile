_default: _just-check
    @{{ just_executable() }} --choose  # Requires 'fzf' (https://formulae.brew.sh/formula/fzf)

_just-check:
    {{ just_executable() }} --unstable --fmt --check

# Updates all the dependencies (except MODULE.bazel)
update:
    cargo update
    bazel run -- @pnpm --dir $(pwd)/sandbox/tauri-hello-world update
    bazel run -- @pnpm --dir $(pwd)/sandbox/svelte-hello-world update

# Run all the tests
test:
    bazel test //...
