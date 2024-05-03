"""Rules associated to Diesel ORM
"""

def _diesel_setup_impl(ctx):
    args = ctx.actions.args()
    args.add("setup")
    args.add("--database-url")
    args.add(ctx.outputs.database_url)
    args.add("--config-file")
    args.add(ctx.file.config_file)

    ctx.actions.run(
        inputs = [ctx.file.config_file],
        outputs = [ctx.outputs.database_url],
        arguments = [args],
        progress_message = "Running Diesel CLI setup",
        executable = ctx.executable._diesel_cli,
    )

diesel_setup = rule(
    implementation = _diesel_setup_impl,
    attrs = {
        "database_url": attr.output(
            doc = "Path to the generated sqlite3 database",
            mandatory = False,
        ),
        "config_file": attr.label(
            allow_single_file = True,
            doc = "Configuration file (diesel.toml)",
        ),
        "_diesel_cli": attr.label(
            default = Label("@diesel_cli//:diesel_cli"),
            allow_single_file = True,
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Executes CLI `diesel setup`",
)
