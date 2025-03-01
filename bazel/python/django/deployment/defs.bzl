"""Rule implementation to make a django deployment"""

def _deploy_local_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)
    # Takes a target directory from the command line

    # Copies the admin, gunicorn, celery,... to the output directory

    # Creates the environment file

    # Creates the supervisord file

    # Creates the nginx file

    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%NAME%": ctx.label.package.replace("/", "_"),
            "%GUNICORN_APP%": "the-gunicorn-app",
            # "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            # "%TOOL%": to_rlocation_path(ctx, tool),
            # "%TOOL_ARGS%": " ".join(args),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([])
    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

deploy_local = rule(
    _deploy_local_impl,
    attrs = {
        "admin": attr.label(
            doc = "Django admin application",
            mandatory = True,
        ),
        "gunicorn": attr.label(
            doc = "Django gunicorn application",
            mandatory = True,
        ),
        "celery": attr.label(
            doc = "Django gunicorn application",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/python/django/deployment:deployment.tpl.sh"),
            allow_single_file = True,
        ),
        # "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    doc = "Deploys the applications to the given folder",
    executable = True,
)
