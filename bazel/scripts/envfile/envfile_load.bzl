"""Generate preinstall script for pkgbuild"""

def _envfile_load_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = {
            "%ENVFILE%": ctx.attr.envfile,
        },
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

envfile_load = rule(
    implementation = _envfile_load_impl,
    attrs = {
        "envfile": attr.string(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/scripts/envfile:envfile_load.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "A rule to create a script to load an envfile",
)
