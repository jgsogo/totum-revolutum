"""
Extensions for rules_qt
"""

load("@rules_qt//:constants.bzl", "DEFAULT_VERSION", "QT_RELEASES", "VERSIONS")
load("@rules_qt//private:install_qt.bzl", "install_qt")

def _qt_impl(ctx):
    qt_version = DEFAULT_VERSION
    for mod in ctx.modules:
        for arg in mod.tags.configure:
            qt_version = arg.qt_version

    for host, target_sdk, arch in QT_RELEASES:
        name = "qt_{}_{}".format(host, arch)
        install_qt(
            name = name,
            host = host,
            version = qt_version,
            arch = arch,
            target_sdk = target_sdk,
        )

_configure = tag_class(attrs = {
    "qt_version": attr.string(values = VERSIONS),
})

qt = module_extension(
    implementation = _qt_impl,
    tag_classes = {
        "configure": _configure,
    },
)
