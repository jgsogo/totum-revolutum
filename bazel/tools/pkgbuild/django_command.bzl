"""Generate script to execute a django command"""

def _django_command_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = {
            "%DJANGO_ADMIN_APP%": ctx.attr.django_admin_app,
            "%COMMAND%": " ".join(ctx.attr.command),
        },
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

django_command = rule(
    implementation = _django_command_impl,
    attrs = {
        "django_admin_app": attr.string(mandatory = True),
        "command": attr.string_list(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/tools/pkgbuild:django_command.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "A rule to execute a django-command",
)
