"""Rule implementation to run Django makemigrations"""

load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

def _gh_release_impl(ctx):
    runfiles = ctx.runfiles(files = [ctx.executable._gh_tool, ctx.file.data])
    runfiles = runfiles.merge(ctx.attr._gh_tool[DefaultInfo].default_runfiles)

    file_path = ctx.file.data.short_path
    if ctx.attr.display_label:
        file_path += "#{}".format(ctx.attr.display_label)

    ctx.actions.write(
        output = ctx.outputs.executable,
        content = "{} --repo={} release upload {} {}".format(ctx.executable._gh_tool.short_path, ctx.attr.repo, ctx.attr.tag, file_path),
        is_executable = True,
    )

    return [
        DefaultInfo(
            executable = ctx.outputs.executable,
            runfiles = runfiles,
        ),
    ]

do_gh_release = rule(
    _gh_release_impl,
    attrs = {
        "_gh_tool": attr.label(
            executable = True,
            cfg = "exec",
            default = "//bazel/tools/gh",
        ),
        "data": attr.label(
            allow_single_file = True,
        ),
        # "clobber"
        "display_label": attr.string(
            doc = "Display label for the uploaded asset",
        ),
        "tag": attr.string(
            doc = "Tag of the target GitHub release",
        ),
        "repo": attr.string(
            default = "jgsogo/totum-revolutum",
        ),
    },
    doc = "Uploads an artifact associated to a release",
    executable = True,
)

def gh_release(name, tag, data, display_label = None, **kwargs):
    pkg_tar(
        name = "{}-files".format(name),
        srcs = data,
    )

    do_gh_release(
        name = name,
        data = ":{}-files".format(name),
        display_label = display_label,
        tag = tag,
        **kwargs
    )
