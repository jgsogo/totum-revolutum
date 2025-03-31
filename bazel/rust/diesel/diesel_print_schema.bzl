"""Generate the schema.rs file
"""

load("@bazel_skylib//rules:diff_test.bzl", "diff_test")
load("//bazel:run_copy_to_workspace.bzl", "run_copy_to_workspace")

def _diesel_print_schema_impl(ctx):
    filename = ctx.attr.schema or "src/{}.rs".format(ctx.attr.name)
    schema_file = ctx.actions.declare_file(filename)

    args = ctx.actions.args()
    args.add(ctx.executable._diesel_cli)
    args.add(ctx.file.database_url)
    args.add(ctx.file.config_file)
    args.add(ctx.file.patch_schema)
    args.add(schema_file)

    ctx.actions.run_shell(
        inputs = [ctx.file.config_file, ctx.file.database_url, ctx.file.patch_schema],
        outputs = [schema_file],
        progress_message = "Running Diesel CLI print-schema",
        arguments = [args],
        command = "$1 print-schema --database-url $2 --config-file $3 --patch-file $4 > $5",
        tools = [ctx.attr._diesel_cli.files_to_run],
    )

    return [DefaultInfo(files = depset([schema_file]))]

_diesel_print_schema = rule(
    implementation = _diesel_print_schema_impl,
    attrs = {
        "database_url": attr.label(
            doc = "Path to the generated sqlite3 database",
            allow_single_file = True,
        ),
        "config_file": attr.label(
            allow_single_file = True,
            doc = "Configuration file (diesel.toml)",
        ),
        "schema": attr.string(
            doc = "Path to the generated schema file",
            mandatory = False,
        ),
        "patch_schema": attr.label(
            allow_single_file = True,
            doc = "A unified diff file to be applied to the final schema.",
            default = Label("//bazel/rust/diesel:skip_rustfmt.patch"),
        ),
        "_diesel_cli": attr.label(
            default = Label("@diesel_cli"),
            allow_single_file = True,
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Executes CLI `diesel print-schema`",
)

def diesel_print_schema(name, target, **kwargs):
    _diesel_print_schema(
        name = name,
        schema = "{}.generated.rs".format(name),
        **kwargs
    )

    run_copy_to_workspace(
        name = "{}.update".format(name),
        origin = ":{}".format(name),
        tags = ["update", "manual"],
        target = target,
    )

    diff_test(
        name = "{}.test".format(name),
        file1 = ":{}".format(name),
        file2 = target,
        failure_message = "\n\nRun `bazel run {}:{}.update` to update the schema file".format(native.package_name(), name),
    )
