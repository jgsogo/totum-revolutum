"""Provides functions to fetch external repositories"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")
load("//bazel/containers/buildx:configure_buildx.bzl", "configure_buildx")

BUILDX_VERSION = "0.21.1"

def _fetch_buildx_impl(_ctx):
    http_file(
        name = "buildx_linux_amd64",
        urls = [
            "https://github.com/docker/buildx/releases/download/v{version}/buildx-v{version}.linux-amd64".format(version = BUILDX_VERSION),
        ],
        integrity = "",
        executable = True,
    )

    http_file(
        name = "buildx_darwin_arm64",
        urls = [
            "https://github.com/docker/buildx/releases/download/v{version}/buildx-v{version}.darwin-arm64".format(version = BUILDX_VERSION),
        ],
        integrity = "",
        executable = True,
    )

    http_file(
        name = "buildx_darwin_amd64",
        urls = [
            "https://github.com/docker/buildx/releases/download/v{version}/buildx-v{version}.darwin-amd64".format(version = BUILDX_VERSION),
        ],
        integrity = "sha256-Lo9FQGLSHyv4d/WzZMItANbDNG+tW4EO/X4c646yZOQ=",
        executable = True,
    )

    configure_buildx(name = "configure_buildx")

    return _ctx.extension_metadata(
        root_module_direct_deps = "all",
        root_module_direct_dev_deps = [],
        reproducible = True,
    )

fetch_buildx = module_extension(
    implementation = _fetch_buildx_impl,
)
