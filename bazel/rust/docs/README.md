The magic in this Bazel module is based on the comments in this issue https://github.com/bazelbuild/rules_rust/issues/1837,
the source code is taken (and adapted) from https://github.com/bazelbuild/rules_rust/compare/main...konkers:rules_rust:wip/rustdoc
with these modifications:
 * Instead of modifying @rules_rust, I'm create the rule in my repository
 * I improved `capture_args.rs` script so it can handle params files (see https://bazel.build/rules/lib/builtins/Args.html),
   basically when there are too many arguments, Bazel spills those to a params file and replace them with a pointer to
   the file. In my `capture_args.rs` version I'm just following the pointer and collecting those arguments as well.
