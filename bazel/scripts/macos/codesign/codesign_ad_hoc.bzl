"""A script to codesign all dylib and frameworks in a directory"""

def _codesign_ad_hoc_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)
    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = {
            "%TARGET_DIR%": ctx.attr.target_directory,
        },
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

codesign_ad_hoc = rule(
    implementation = _codesign_ad_hoc_impl,
    attrs = {
        "target_directory": attr.string(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/scripts/macos/codesign:codesign_ad_hoc.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "A script to codesign all dylib and frameworks in a directory",
)
