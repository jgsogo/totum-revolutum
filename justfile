set dotenv-load := true

_default: _just-check
    @{{ just_executable() }} --choose  # Requires 'fzf' (https://formulae.brew.sh/formula/fzf)

_just-check:
    {{ just_executable() }} --unstable --fmt --check

# Updates all the dependencies (MODULE.bazel and 3rd parties not included)
update: update-deps update-precommit

update-precommit:
    pre-commit autoupdate

# Updates only the dependencies
update-deps: update-rust update-npm update-python

update-rust:
    cargo update
    bazel run //bazel/third_party:crates_vendor

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
    # FIXME: Remove. These 'install' rules are just creating the node_modules in the workspace, but Bazel uses the ones in the build directory (created by the 'npm_link_all_packages' rule)
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

###
# GH self-hosted runners
###

bazel-remote:
    docker-compose --env-file .env -f ./tools/github/self-hosted-runner/docker-compose-bazel.yml up --build -d bazel-remote

# Run bazel-remote (cache) and github runner
gh-runner-linux:
    docker-compose --env-file .env -f ./tools/github/self-hosted-runner/docker-compose-bazel.yml up --build -d

gh-runner-linux-logs:
    docker-compose -f ./tools/github/self-hosted-runner/docker-compose-bazel.yml logs -f

# Stops bazel-remote (cache) and github runner
gh-runner-linux-stop:
    docker-compose -f ./tools/github/self-hosted-runner/docker-compose-bazel.yml down

# Run gh self-hosted runner for Macos
[working-directory('actions-runner')]
gh-runner-macos:
    curl -o actions-runner-osx-x64-2.322.0.tar.gz -L https://github.com/actions/runner/releases/download/v2.322.0/actions-runner-osx-x64-2.322.0.tar.gz
    echo "aa0fc262363912167dcdbc746ffcdf7b8996bc587f51cf1bab38ad86cf70b6ea  actions-runner-osx-x64-2.322.0.tar.gz" | shasum -a 256 -c
    tar xzf ./actions-runner-osx-x64-2.322.0.tar.gz
    rm actions-runner-osx-x64-2.322.0.tar.gz
    ./../tools/github/self-hosted-runner/runner.sh # Do not detach

# Run self-hosted runners for Linux and Macos
gh-runner: gh-runner-linux gh-runner-macos

# Stop self-hosted runners for Linux
gh-runner-stop: gh-runner-linux-stop
