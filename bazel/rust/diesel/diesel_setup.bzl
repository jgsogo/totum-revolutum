"""Rules associated to Diesel ORM
"""

def _diesel_setup_impl(ctx):
    # Declare a new directory. All migrations will be copied here
    migration_dir = ctx.actions.declare_directory(ctx.label.name + "-migrations")

    # Collect all migrations (deduplicate)
    all_migrations = []
    for it in ctx.files.migrations:
        all_migrations.append(it.dirname)
    all_migrations = depset(all_migrations).to_list()

    # Create and copy inside this directory all the migrations
    args = ctx.actions.args()
    args.add_all(all_migrations)
    ctx.actions.run_shell(
        inputs = ctx.files.migrations,
        outputs = [migration_dir],
        arguments = [args],
        command = "mkdir -p {} && cp -r $@ {}".format(migration_dir.path, migration_dir.path),
    )

    # Now execute the migrations that were copied to the folder
    args = ctx.actions.args()
    args.add("setup")
    args.add("--database-url")
    args.add(ctx.outputs.database_url)
    args.add("--config-file")
    args.add(ctx.file.config_file)
    args.add("--migration-dir")
    args.add(migration_dir.path)

    ctx.actions.run(
        inputs = [ctx.file.config_file, migration_dir],
        outputs = [ctx.outputs.database_url],
        arguments = [args],
        progress_message = "Running Diesel CLI setup (migrations)",
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
        "migrations": attr.label_list(
            allow_files = True,
            doc = "All the migrations to be applied",
            mandatory = False,
        ),
        "_diesel_cli": attr.label(
            default = Label("@diesel_cli"),
            allow_single_file = True,
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Executes diesel-cli to create the DB applying all the given migrations.",
)
