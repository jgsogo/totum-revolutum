"""Generate preinstall script for pkgbuild"""

def _combine_envfiles_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = {
            "%SOURCE%": ctx.attr.source,
            "%TARGET%": ctx.attr.target,
        },
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

combine_envfiles = rule(
    implementation = _combine_envfiles_impl,
    attrs = {
        "source": attr.string(mandatory = True),
        "target": attr.string(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/tools/pkgbuild/scripts/envfile:combine_envfiles.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "A rule to combine two env-files. It will override the target_file with the content of the source_file, while preserving the values that are already in the target_file",
)
