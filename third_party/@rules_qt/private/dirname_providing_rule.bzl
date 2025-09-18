"""
A rule to provide dirname, taken from https://www.stevenengelhardt.com/2023/03/03/practical-bazel-custom-bazel-make-variables/
"""

load("@bazel_skylib//lib:paths.bzl", "paths")

def _impl(ctx):
    return [
        platform_common.TemplateVariableInfo({
            ctx.attr.varname: paths.dirname(ctx.expand_location(ctx.attr.value, ctx.attr.data)) + ctx.attr.append,
        }),
    ]

dirname_providing_rule = rule(
    implementation = _impl,
    attrs = {
        "varname": attr.string(mandatory = True),
        "value": attr.string(mandatory = True),
        "data": attr.label_list(allow_files = True),
        "append": attr.string(),
    },
)
