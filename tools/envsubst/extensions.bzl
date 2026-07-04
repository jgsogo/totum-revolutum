"""Provides module to fetch envsubst application"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

VERSION = "1.4.2"

# FIXME: We may use https://github.com/theoremlp/rules_multitool for this (and other tools-like applications)
def _fetch_envsubst_impl(_ctx):
    http_file(
        name = "envsubst_linux_arm64",
        urls = [
            "https://github.com/a8m/envsubst/releases/download/v{version}/envsubst-Linux-arm64".format(version = VERSION),
        ],
        integrity = "sha256-cBuUAkerO2+Z0d4LIZnsgIKPkJItRns2+Yuor0yZjkg=",
        executable = True,
    )

    http_file(
        name = "envsubst_darwin_arm64",
        urls = [
            "https://github.com/a8m/envsubst/releases/download/v{version}/envsubst-Darwin-arm64".format(version = VERSION),
        ],
        integrity = "sha256-0gnNnlzBEQvSDwNDBJWyw9k6elHj+oIJPTifqdRHHpo=",
        executable = True,
    )

    http_file(
        name = "envsubst_darwin_x86_64",
        urls = [
            "https://github.com/a8m/envsubst/releases/download/v{version}/envsubst-Darwin-x86_64".format(version = VERSION),
        ],
        integrity = "sha256-BzvJT6Tloa9weDAKHlMS6mO3UMJbPOjXdiEWQOBUFVM=",
        executable = True,
    )

    return _ctx.extension_metadata(
        root_module_direct_deps = "all",
        root_module_direct_dev_deps = [],
        reproducible = True,
    )

fetch_envsubst = module_extension(
    implementation = _fetch_envsubst_impl,
)
