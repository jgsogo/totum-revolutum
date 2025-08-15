"""
Third party dependencies
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def tl_expected_repository():
    http_archive(
        name = "tl-expected",
        url = "https://github.com/TartanLlama/expected/archive/refs/tags/v1.2.0.tar.gz",
        build_file = Label("//third_party/tl-expected:tl-expected.BUILD"),
        strip_prefix = "expected-1.2.0",
    )

def _non_module_dependencies_impl(_ctx):
    tl_expected_repository()

non_module_dependencies = module_extension(
    implementation = _non_module_dependencies_impl,
)
