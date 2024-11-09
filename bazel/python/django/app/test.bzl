"""Rule implementation to run Django tests"""

def _django_test_impl(ctx):
    runfiles = ctx.runfiles(files = [ctx.executable.django_admin_tool] + ctx.files.srcs)
    runfiles = runfiles.merge(ctx.attr.django_admin_tool[DefaultInfo].default_runfiles)

    ctx.actions.write(
        output = ctx.outputs.executable,
        content = "{} test {}".format(ctx.executable.django_admin_tool.short_path, ctx.attr.test_label),
        is_executable = True,
    )

    return [
        DefaultInfo(
            executable = ctx.outputs.executable,
            runfiles = runfiles,
        ),
    ]

django_test = rule(
    _django_test_impl,
    attrs = {
        "test_label": attr.string(
            doc = "Module paths to test; can be modulename, modulename.TestCase or modulename.TestCase.test_method",
            default = "tests",
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
    doc = "Executes the tests for a given Django application (within a project).",
    test = True,
)
