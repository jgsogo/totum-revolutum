# """
# Extensions for rules_qt
# """

# load("@rules_qt//tools/aqt:install_qt.bzl", "install_qt")
# load("@rules_qt//:constants.bzl", "VERSIONS")
# load("@rules_qt//private:utils.bzl", "get_repo_name")

# def _all_repos(ctx):
#     for version in VERSIONS:

#     qt.install(
#     name = "qt_6.10.0_mac_x86_64",
#     arch = "clang_64",
#     host = "mac",
#     target_sdk = "desktop",
#     version = "6.10.0",
# )

#         install_qt(
#             name = arg.name,
#             host = arg.host,
#             version = arg.version,
#             arch = arg.arch,
#             target_sdk = arg.target_sdk,
#         )

#     for mod in ctx.modules:
#         if mod.name == "rules_qt":
#             for arg in mod.tags.install:
#                 install_qt(
#                     name = arg.name,
#                     host = arg.host,
#                     version = arg.version,
#                     arch = arg.arch,
#                     target_sdk = arg.target_sdk,
#                 )

# _install = tag_class(attrs = {
#     "name": attr.string(),
#     "host": attr.string(),
#     "version": attr.string(),
#     "arch": attr.string(),
#     "target_sdk": attr.string(),
# })

# qt_private = module_extension(
#     implementation = _all_repos,
# )

# def tl_expected_repository():
#     http_archive(
#         name = "tl-expected",
#         url = "https://github.com/TartanLlama/expected/archive/refs/tags/v1.2.0.tar.gz",
#         build_file = Label("//third_party/tl-expected:tl-expected.BUILD"),
#         strip_prefix = "expected-1.2.0",
#     )

# def _non_module_dependencies_impl(_ctx):
#     tl_expected_repository()

# non_module_dependencies = module_extension(
#     implementation = _non_module_dependencies_impl,
# )
