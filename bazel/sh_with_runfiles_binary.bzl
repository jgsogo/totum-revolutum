"""Diesel print-schema CLI command
"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

def _sh_with_runfiles_binary_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    args = []
    for it in ctx.attr.args:
        arg = ctx.expand_location(it, targets = ctx.attr.data)
        args.append(arg.replace("$$", "$"))

    env_file = ctx.actions.declare_file(ctx.label.name + ".env")
    env_file_content = []
    for key, value in ctx.attr.env.items():
        value = ctx.expand_location(value, targets = ctx.attr.data)
        env_file_content.append("{}={}".format(key, value))
    ctx.actions.write(
        output = env_file,
        content = "\n".join(env_file_content),
    )

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%TOOL%": to_rlocation_path(ctx, ctx.attr.tool.files_to_run.executable),
            "%TOOL_ARGS%": " ".join(args),
            "%ENV_FILE%": env_file.short_path,
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([env_file], transitive_files = depset([]), collect_data = True)
    runfiles = runfiles.merge(ctx.attr.tool.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.tool[DefaultInfo].default_runfiles)
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
            # allow_single_file = True,
            executable = True,
            cfg = "target",
            mandatory = True,
        ),
        "data": attr.label_list(
            allow_files = True,
        ),
        "env": attr.string_dict(
            doc = "Environment variables",
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
