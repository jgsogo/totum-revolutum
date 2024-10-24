"""Rule implementation to run Django tests"""

def _django_makemigrations_impl(ctx):
    # runfiles = ctx.runfiles(files = [ctx.executable.django_admin_tool] + ctx.files.srcs)
    # runfiles = runfiles.merge(ctx.attr.django_admin_tool[DefaultInfo].default_runfiles)

    # ctx.actions.write(
    #     output = ctx.outputs.executable,
    #     content = "{} makemigrations {}".format(ctx.executable.django_admin_tool.short_path, ctx.attr.app_label),
    #     is_executable = True,
    # )

    output = ctx.actions.declare_directory("fixtures")

    args = ctx.actions.args()
    args.add("makemigrations")
    args.add(ctx.attr.app_label)

    ctx.actions.run(
        # inputs = [ctx.file.config_file, migration_dir],
        # outputs = [ctx.outputs.database_url],
        outputs = [output],
        arguments = [args],
        progress_message = "Running Django makemigrations",
        executable = ctx.executable.django_admin_tool,
    )

    # return [
    #     DefaultInfo(
    #         executable = ctx.outputs.executable,
    #         runfiles = runfiles,
    #     ),
    # ]

django_makemigrations = rule(
    _django_makemigrations_impl,
    attrs = {
        "app_label": attr.string(
            doc = "Name of the application",
            mandatory = True,
        ),
        "srcs": attr.label_list(
            allow_files = True,
        ),
        "django_admin_tool": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Creates migrations for the given Django application",
)
