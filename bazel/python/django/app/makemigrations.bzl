"""Rule implementation to run Django makemigrations"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

def _django_makemigrations_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    update_rule_str = "<not-provided>"
    if ctx.attr.update_rule:
        update_rule_label = Label(ctx.attr.update_rule.label)
        update_rule_str = "//" + update_rule_label.package + ":" + update_rule_label.name

    ctx.actions.expand_template(
        template = ctx.file.run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%UPDATE_RULE%": update_rule_str,
            "%DJANGO_ADMIN%": to_rlocation_path(ctx, ctx.executable.django_admin_tool),
            "%APP_LABEL%": ctx.attr.app_label,
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles()
    runfiles = runfiles.merge(ctx.attr.django_admin_tool.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.django_admin_tool[DefaultInfo].default_runfiles)
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

django_makemigrations = rule(
    _django_makemigrations_impl,
    attrs = {
        "app_label": attr.string(
            doc = "Name of the application",
            mandatory = True,
        ),
        "django_admin_tool": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
        ),
        "run_template": attr.label(
            allow_single_file = True,
        ),
        "update_rule": attr.label(),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    doc = "Executes makemigrations Django command",
    executable = True,
)

def django_makemigrations_update(name, *args, **kwargs):
    """Executes makemigrations and copies the generated migrations into the workspace"""
    django_makemigrations(
        name = name,
        run_template = "//bazel/python/django/app:makemigrations.update.tpl.sh",
        *args,
        **kwargs
    )

def django_makemigrations_check(name, *args, **kwargs):
    """Executes makemigrations testing if there is anything pending"""
    django_makemigrations(
        name = name,
        run_template = "//bazel/python/django/app:makemigrations.check.tpl.sh",
        *args,
        **kwargs
    )
