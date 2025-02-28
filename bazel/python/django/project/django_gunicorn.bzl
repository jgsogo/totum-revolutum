"""Rule implementation to run gunicorn"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")
load("@aspect_rules_py//py:defs.bzl", "py_binary")
load("@py_deps//:requirements.bzl", "requirement")

def _django_gunicorn_sh_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%GUNICORN%": to_rlocation_path(ctx, ctx.attr.gunicorn_binary.files_to_run.executable),
            "%SETTINGS%": ctx.attr.settings,
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([])
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.gunicorn_binary.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
        RunEnvironmentInfo(
            environment = {
                "GUNICORN_APP_NAME": ctx.label.package.replace("/", "_"),
                "GUNICORN_WORKING_DIR": "/tmp/{}".format(ctx.label.package.replace("/", "_")),
            },
        ),
    ]

django_gunicorn_sh = rule(
    _django_gunicorn_sh_impl,
    attrs = {
        "gunicorn_binary": attr.label(
            doc = "This is the gunicorn executable",
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
            default = Label("//bazel/python/django/project:django_gunicorn.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    doc = "Creates a shell script to run gunicorn with all the configuration",
    executable = True,
)

def django_gunicorn(name, django_project, **kwargs):
    # Django project plus the gunicorn
    py_binary(
        name = "{}-bin".format(name),
        srcs = ["//bazel/python/gunicorn:gunicorn_wrapper.py"],
        main = "//bazel/python/gunicorn:gunicorn_wrapper.py",
        deps = [
            django_project,
            requirement("gunicorn"),
        ],
    )

    # Shell script to execute it
    django_gunicorn_sh(
        name = name,
        gunicorn_binary = ":{}-bin".format(name),
        **kwargs
    )
