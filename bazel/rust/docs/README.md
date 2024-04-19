The magic in this Bazel module is based on the comments in this issue https://github.com/bazelbuild/rules_rust/issues/1837,
the source code is taken (and adapted) from https://github.com/bazelbuild/rules_rust/compare/main...konkers:rules_rust:wip/rustdoc
with these modifications:
 * Instead of modifying @rules_rust, I'm create the rule in my repository
 * I improved `capture_args.rs` script so it can handle params files (see https://bazel.build/rules/lib/builtins/Args.html),
   basically when there are too many arguments, Bazel spills those to a params file and replace them with a pointer to
   the file. In my `capture_args.rs` version I'm just following the pointer and collecting those arguments as well.


# TODO: Not implemented yet

There are still some missing bits:
 * The rule captures the command line arguments, but it doesn't capture (and forward) the environment variables. Some
   of these variables are required at compile time (uses `env!`). We need `capture_args` to capture environment variables
   too and then reuse them in `run_scripts.rs`.
