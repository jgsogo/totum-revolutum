"""
A repository rule to install Qt
"""

load("@rules_qt//private:utils.bzl", "get_build_filename")

def _install_qt_impl(rctx):
    # See if aqt is installed
    # TODO: Use, somehow, the Bazel target
    r = rctx.execute(["command", "-v", "aqt"])
    if r.return_code != 0:
        fail("Install aqt first: 'pip install aqt'")

    host = rctx.attr.host
    version = rctx.attr.version
    arch = rctx.attr.arch
    target_sdk = rctx.attr.target_sdk

    r = rctx.execute(["aqt", "install-qt", host, target_sdk, version, arch])
    if r.return_code != 0:
        fail("Failed to install Qt version: \nSTDERR:\n{}\nSTDOUT:\n{}".format(r.stderr, r.stdout))

    # TODO: We could remove directories that we won't use
    build_filename = get_build_filename("qt", version, host, arch, target_sdk)

    rctx.file("MODULE.bazel", content = "module(name = {})".format(rctx.attr.name))
    build_file = Label("@rules_qt//data:{}".format(build_filename))
    rctx.file("BUILD.bazel", content = rctx.read(build_file))

install_qt = repository_rule(
    implementation = _install_qt_impl,
    attrs = {
        "host": attr.string(),
        "version": attr.string(),
        "arch": attr.string(),
        "target_sdk": attr.string(),
    },
)
