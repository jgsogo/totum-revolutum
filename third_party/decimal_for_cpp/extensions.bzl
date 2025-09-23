"""
Third party dependencies
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

COMMIT = "599372ee214ab37b5c0fc68148352321978f20ed"

def decimal_for_cpp_repository():
    http_archive(
        name = "decimal_for_cpp",
        url = "https://github.com/vpiotr/decimal_for_cpp/archive/{}.tar.gz".format(COMMIT),
        build_file = Label("//third_party/decimal_for_cpp:decimal_for_cpp.BUILD"),
        strip_prefix = "decimal_for_cpp-{}".format(COMMIT),
        patches = [
            "//third_party/decimal_for_cpp:0001-pessimizing-move.patch",
        ],
        patch_args = [
            "-p1",
        ],
    )

def _non_module_dependencies_impl(_ctx):
    decimal_for_cpp_repository()

non_module_dependencies = module_extension(
    implementation = _non_module_dependencies_impl,
)
