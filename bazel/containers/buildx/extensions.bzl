"""Provides functions to fetch external repositories"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")
load("//bazel/containers/buildx:configure_buildx.bzl", "configure_buildx")

BUILDX_VERSION = "0.14.0"

def _fetch_buildx_impl(_ctx):
    http_file(
        name = "buildx_linux_amd64",
        urls = [
            "https://github.com/docker/buildx/releases/download/v{version}/buildx-v{version}.linux-amd64".format(version = BUILDX_VERSION),
        ],
        integrity = "sha256-Mvjxfso1vy7+bA5H9A5Gkqh280UxtCHvyYR5mltBIm4=",
        executable = True,
    )

    http_file(
        name = "buildx_darwin_arm64",
        urls = [
            "https://github.com/docker/buildx/releases/download/v{version}/buildx-v{version}.darwin-arm64".format(version = BUILDX_VERSION),
        ],
        integrity = "sha256-3BdvI2ZgnMITKubwi7IZOjL5/ZNUv9Agz3+juNt0hA0=",
        executable = True,
    )

    http_file(
        name = "buildx_darwin_amd64",
        urls = [
            "https://github.com/docker/buildx/releases/download/v{version}/buildx-v{version}.darwin-amd64".format(version = BUILDX_VERSION),
        ],
        integrity = "sha256-J6rZfENSvCzFBHDgnA8Oqq2FDXR+M9CTejhhg9DruPU=",
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
