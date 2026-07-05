"""Provides functions to fetch external repositories"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

GH_VERSION = "2.61.0"

# FIXME: We may use https://github.com/theoremlp/rules_multitool for this (and other tools-like applications)
def _fetch_gh_impl(_ctx):
    http_archive(
        name = "gh_linux_amd64",
        urls = [
            "https://github.com/cli/cli/releases/download/v{version}/gh_{version}_linux_amd64.tar.gz".format(version = GH_VERSION),
        ],
        type = "tar.gz",
        sha256 = "e2fe1a63cef003093eb1f8e4a669e9e763bd0b747de8abb3253411b408ef6ede",
        strip_prefix = "gh_{version}_linux_amd64".format(version = GH_VERSION),
        build_file = "//bazel/tools/gh:BUILD.gh.bazel",
    )

    http_archive(
        name = "gh_darwin_arm64",
        urls = [
            "https://github.com/cli/cli/releases/download/v{version}/gh_{version}_macOS_arm64.zip".format(version = GH_VERSION),
        ],
        type = "zip",
        sha256 = "1489f762f2e0d49aa5fd8a7cf652faae5f8ac9212612b78c3180cbd88d00db25",
        strip_prefix = "gh_{version}_macOS_arm64".format(version = GH_VERSION),
        build_file = "//bazel/tools/gh:BUILD.gh.bazel",
    )

    http_archive(
        name = "gh_darwin_amd64",
        urls = [
            "https://github.com/cli/cli/releases/download/v{version}/gh_{version}_macOS_amd64.zip".format(version = GH_VERSION),
        ],
        type = "zip",
        sha256 = "794ab3aa580edf3849ff5a4f11b3e5179953958d97a6b71d7ca6fddc043c8e43",
        strip_prefix = "gh_{version}_macOS_amd64".format(version = GH_VERSION),
        build_file = "//bazel/tools/gh:BUILD.gh.bazel",
    )

    return _ctx.extension_metadata(
        root_module_direct_deps = "all",
        root_module_direct_dev_deps = [],
        reproducible = True,
    )

fetch_gh = module_extension(
    implementation = _fetch_gh_impl,
)
