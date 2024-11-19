"""Rule implementation to run Django tests"""

def _django_migrate_impl(ctx):
    runfiles = ctx.runfiles(files = [ctx.executable.django_admin_tool])
    runfiles = runfiles.merge(ctx.attr.django_admin_tool[DefaultInfo].default_runfiles)

    ctx.actions.write(
        output = ctx.outputs.executable,
        content = "{} migrate".format(ctx.executable.django_admin_tool.short_path),
        is_executable = True,
    )

    return [
        DefaultInfo(
            executable = ctx.outputs.executable,
            runfiles = runfiles,
        ),
    ]

django_migrate = rule(
    _django_migrate_impl,
    attrs = {
        "django_admin_tool": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Executes the tests for a given Django application (within a project).",
    executable = True,
)
