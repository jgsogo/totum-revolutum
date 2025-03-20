"""Generate a script to create and optionally CHOWN the symlink"""

def _create_symlink_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    substitutions = {
        "%SOURCE%": ctx.attr.source,
        "%TARGET%": ctx.attr.target,
    }

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = substitutions,
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

create_symlink = rule(
    implementation = _create_symlink_impl,
    attrs = {
        "source": attr.string(mandatory = True),
        "target": attr.string(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/scripts/bash:create_symlink.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "Instantiates a script that creates a symlink",
)
