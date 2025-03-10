"""Generate preinstall script for pkgbuild"""

def _untar_files_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = {
            "%FILES_TO_WORK_ON%": " ".join(ctx.attr.files),
        },
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

untar_files = rule(
    implementation = _untar_files_impl,
    attrs = {
        "files": attr.string_list(mandatory = True),
        "_template": attr.label(
            default = Label("//bazel/tools/pkgbuild/scripts/filesystem:untar_files.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "A rule to create a script that will untar a list of files",
)
