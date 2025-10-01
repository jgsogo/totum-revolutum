"""
Third party dependencies
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def date_repository():
    http_archive(
        name = "date",
        url = "https://github.com/HowardHinnant/date/archive/refs/tags/v3.0.4.tar.gz",
        build_file = Label("//third_party/date:date.BUILD"),
        strip_prefix = "date-3.0.4",
    )

def _non_module_dependencies_impl(_ctx):
    date_repository()

non_module_dependencies = module_extension(
    implementation = _non_module_dependencies_impl,
)
