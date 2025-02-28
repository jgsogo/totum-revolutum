"""Rule implementation to run django-admin"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")
load("@aspect_rules_py//py:defs.bzl", "py_binary")

def _django_admin_sh_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%DJANGO_ADMIN%": to_rlocation_path(ctx, ctx.attr.project.files_to_run.executable),
            "%SETTINGS%": ctx.attr.settings,
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([])
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.project.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
        # RunEnvironmentInfo(
        #     environment = {},
        # ),
    ]

django_admin_sh = rule(
    _django_admin_sh_impl,
    attrs = {
        "project": attr.label(
            doc = "This is the django admin executable",
            # providers = [PyInfo],  # TODO: Maybe a DjangoProvider that provides the python project and the settings
            mandatory = True,
            executable = True,
            cfg = "target",
        ),
        "settings": attr.string(
            doc = "The settings module",
            mandatory = True,
        ),
        "_run_template": attr.label(
            default = Label("//bazel/python/django/project:django_admin.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    doc = "Creates a shell script to run the django-admin with all the configuration",
    executable = True,
)

def django_admin(name, django_project, **kwargs):
    py_binary(
        name = "{}-bin".format(name),
        srcs = ["//bazel/python/django/project:manage.py"],
        main = "manage.py",
        deps = [
            django_project,
        ],
    )

    django_admin_sh(
        name = name,
        project = ":{}-bin".format(name),
        **kwargs
    )
