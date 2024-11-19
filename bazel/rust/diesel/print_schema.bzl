"""Diesel print-schema CLI command
"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

def _diesel_print_schema_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    diesel_cli_args = []
    if ctx.attr.only_tables:
        diesel_cli_args.append("--only-tables \"{}\"".format(ctx.attr.only_tables))

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%DIESEL_CLI%": to_rlocation_path(ctx, ctx.file._diesel_cli),
            "%SCHEMA_FILE%": ctx.attr.schema,
            "%DIESEL_CLI_ARGS%": " ".join(diesel_cli_args),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([ctx.file._diesel_cli], transitive_files = depset([]))
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

diesel_print_schema = rule(
    implementation = _diesel_print_schema_impl,
    attrs = {
        "config_file": attr.label(
            allow_single_file = True,
            doc = "Configuration file (diesel.toml)",
        ),
        "schema": attr.string(
            doc = "Path to the generated schema file",
            mandatory = False,
        ),
        "only_tables": attr.string(
            doc = "Only include tables from table-name that matches regexp.",
        ),
        "patch_schema": attr.label(
            allow_single_file = True,
            doc = "A unified diff file to be applied to the final schema.",
            default = Label("//bazel/rust/diesel:skip_rustfmt.patch"),
        ),
        "_diesel_cli": attr.label(
            default = Label("@diesel_cli//:diesel_cli"),
            allow_single_file = True,
            executable = True,
            cfg = "exec",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/rust/diesel:print_schema.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    executable = True,
    doc = "Executes CLI `diesel print-schema`",
)
