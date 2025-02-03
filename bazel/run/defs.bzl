"""Provides a rule to run to executables in parallel"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

def _run_together_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    all_files = [ctx.executable.before, ctx.executable.after]

    after = ctx.attr.after.files_to_run.executable
    before = ctx.attr.before.files_to_run.executable

    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%{after_executable}": to_rlocation_path(ctx, after),
            "%{before_executable}": to_rlocation_path(ctx, before),
            "%{sleep}": str(ctx.attr.delay_seconds),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = all_files, transitive_files = depset([]), collect_data = True)
    runfiles = runfiles.merge(ctx.attr.after.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.before.default_runfiles)
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    # Environment
    environment = {}
    if RunEnvironmentInfo in ctx.attr.before:
        environment = environment | ctx.attr.before[RunEnvironmentInfo].environment
    if RunEnvironmentInfo in ctx.attr.after:
        environment = environment | ctx.attr.after[RunEnvironmentInfo].environment

    # Transitive dependencies:
    # my_runfiles = my_runfiles.merge(ctx.attr.test[DefaultInfo].default_runfiles)
    # for data in ctx.attr.data:
    #     my_runfiles = my_runfiles.merge(data[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            files = depset(all_files),
            runfiles = runfiles,
            executable = executable,
        ),
        RunEnvironmentInfo(
            environment = environment,
        ),
    ]

run_together = rule(
    doc = """A rule to run together two executables. The first executable will run detached and the second
          will start after a configurable delay. When the process is finished, the PID corresponding
          to the first executable will be killed.

          Note that all the RunEnvironmentInfo providers will be merged
          """,
    attrs = {
        "before": attr.label(
            doc = "The executable to run before",
            executable = True,
            cfg = "target",
        ),
        "after": attr.label(
            doc = "The executable to run after",
            executable = True,
            cfg = "target",
        ),
        "delay_seconds": attr.int(
            doc = "The delay before running the second/'after' executable",
            default = 0,
        ),
        "_run_template": attr.label(
            default = Label("//bazel/run:run_together.sh.tpl"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    implementation = _run_together_impl,
    executable = True,
)
