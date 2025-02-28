"""Rule implementation to run gunicorn"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")
load("@rules_python//python/entry_points:py_console_script_binary.bzl", "py_console_script_binary")

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
        # This environment is only available when using `bazel run <this-rule>`
        RunEnvironmentInfo(
            environment = {
                "GUNICORN_APP_NAME": ctx.label.package.replace("/", "_"),
                "GUNICORN_WORKING_DIR": "/tmp/{}".format(ctx.label.package.replace("/", "_")),
                "GUNICORN_BIND": "localhost:0",
                "DEBUG": "1",
                "SECRET_KEY": "the-secret-key",
                "GUNICORN_EXTRA_ARGS": "--capture-output --error-logfile - --access-logfile -",
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
    py_console_script_binary(
        name = "gunicorn",
        pkg = "@py_deps//gunicorn",
        deps = [
            django_project,
        ],
        tags = ["manual"],
    )

    # Shell script to execute it
    django_gunicorn_sh(
        name = name,
        gunicorn_binary = ":gunicorn",
        **kwargs
    )
