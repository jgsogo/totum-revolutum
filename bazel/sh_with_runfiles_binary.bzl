"""Diesel print-schema CLI command
"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

def _sh_with_runfiles_binary_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    args = []
    for it in ctx.attr.args:
        args.append(it.replace("$$", "$"))

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%TOOL%": to_rlocation_path(ctx, ctx.file.tool),
            "%TOOL_ARGS%": " ".join(args),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([ctx.file.tool], transitive_files = depset([]))
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

sh_with_runfiles_binary = rule(
    implementation = _sh_with_runfiles_binary_impl,
    attrs = {
        # "args": attr.string_list(),
        "tool": attr.label(
            allow_single_file = True,
            executable = True,
            cfg = "exec",
            mandatory = True,
        ),
        "_run_template": attr.label(
            default = Label("//bazel:sh_with_runfiles_binary.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    executable = True,
    doc = "Executes a tool using the Bash Runfiles library to locate it",
)
