"""Generate a script to create and optionally CHOWN the symlink"""

def _chown_folder_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    substitutions = {
        "%FOLDER%": ctx.attr.folder,
    }

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = substitutions,
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

chown_folder = rule(
    implementation = _chown_folder_impl,
    attrs = {
        "folder": attr.string(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/scripts/bash:chown_folder.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "Changes folder owner to $INSTALLER_USER:admin",
)
