"""Generate preinstall script for pkgbuild"""

def _composable_script_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr._output)

    prelude = ctx.actions.declare_file(ctx.label.name + "-prelude")
    ctx.actions.expand_template(
        template = ctx.file._prelude,
        output = prelude,
        substitutions = {
            "%NAME%": ctx.attr.pkgbuild_name,
        },
    )

    ending = ctx.actions.declare_file(ctx.label.name + "-ending")
    ctx.actions.expand_template(
        template = ctx.file._ending,
        output = ending,
        substitutions = {
            "%NAME%": ctx.attr.pkgbuild_name,
        },
    )

    chunks_files = [prelude.path] + [f.path for f in ctx.files.chunks] + [ending.path]

    ctx.actions.run_shell(
        inputs = ctx.files.chunks + [prelude, ending],
        outputs = [output],
        command = "cat {files} > {output}".format(files = " ".join(chunks_files), output = output.path),
    )

    return [
        DefaultInfo(files = depset([output])),
    ]

preinstall = rule(
    implementation = _composable_script_impl,
    attrs = {
        "pkgbuild_name": attr.string(mandatory = True),
        "chunks": attr.label_list(mandatory = True, allow_files = True),
        "_prelude": attr.label(
            default = Label("//bazel/tools/pkgbuild:preinstall_prelude.sh"),
            allow_single_file = True,
        ),
        "_ending": attr.label(
            default = Label("//bazel/tools/pkgbuild:preinstall_ending.sh"),
            allow_single_file = True,
        ),
        "_output": attr.string(default = "preinstall"),
    },
    doc = "A rule to create a preinstall script",
)

postinstall = rule(
    implementation = _composable_script_impl,
    attrs = {
        "pkgbuild_name": attr.string(mandatory = True),
        "chunks": attr.label_list(mandatory = True, allow_files = True),
        "_prelude": attr.label(
            default = Label("//bazel/tools/pkgbuild:postinstall_prelude.sh"),
            allow_single_file = True,
        ),
        "_ending": attr.label(
            default = Label("//bazel/tools/pkgbuild:postinstall_ending.sh"),
            allow_single_file = True,
        ),
        "_output": attr.string(default = "postinstall"),
    },
    doc = "A rule to create a postinstall script",
)
