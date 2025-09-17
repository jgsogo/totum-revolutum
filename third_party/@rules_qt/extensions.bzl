"""
Extensions for rules_qt
"""

load("@rules_qt//tools/aqt:install_qt.bzl", "install_qt")

def _qt_impl(ctx):
    for mod in ctx.modules:
        if mod.name == "rules_qt":
            for arg in mod.tags.install:
                install_qt(
                    name = arg.name,
                    host = arg.host,
                    version = arg.version,
                    arch = arg.arch,
                    target_sdk = arg.target_sdk,
                )

_install = tag_class(attrs = {
    "name": attr.string(),
    "host": attr.string(),
    "version": attr.string(),
    "arch": attr.string(),
    "target_sdk": attr.string(),
})

qt = module_extension(
    implementation = _qt_impl,
    tag_classes = {"install": _install},
)
