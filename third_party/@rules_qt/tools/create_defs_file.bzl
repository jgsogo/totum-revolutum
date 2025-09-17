"""
A function to create the BUILD file for a repo
"""

load("@aspect_bazel_lib//lib:run_binary.bzl", "run_binary")
load("//tools:update_ws_file.bzl", "update_ws_file")
load("//tools/aqt:install_qt.bzl", "get_build_filename")

HOSTS = ["mac"]
TARGET_SDKS = ["desktop"]
ARCHS = ["clang_64"]

def create_defs_file(name, host, target_sdk, version, arch):
    """
    Generates the BUILD file for a repo

    Args:
        name:
        host:
        target_sdk:
        version:
        arch:
    """

    if host not in HOSTS:
        fail("Invalid host value '{}'. Valid hosts are '{}'".format(host, "', '".join(HOSTS)))

    if target_sdk not in TARGET_SDKS:
        fail("Invalid target_sdk value '{}'. Valid target_sdks are '{}'".format(target_sdk, "', '".join(TARGET_SDKS)))

    if arch not in ARCHS:
        fail("Invalid arch value '{}'. Valid archs are '{}'".format(arch, "', '".join(ARCHS)))

    out_dirs_host = {"mac": "macos"}.get(host)

    base_name = "qt_{}_{}".format(version.replace(".", "_"), host)

    run_binary(
        name = base_name,
        tool = "//tools/aqt",
        args = [
            "install-qt",
            host,
            target_sdk,
            version,
            arch,
            "--outputdir $(RULEDIR)",
        ],
        out_dirs = ["{}/{}".format(version, out_dirs_host)],
        tags = ["manual"],
    )

    run_binary(
        name = "{}/deps".format(base_name),
        tool = "//tools/deps",
        args = [
            "--input_path=$(location :{})".format(base_name),
            "--base_path=$(RULEDIR)",
            "--output=$@",
            "--host={}".format(host),
            "--target_sdk={}".format(target_sdk),
            "--version={}".format(version),
            "--arch={}".format(arch),
        ],
        srcs = [":{}".format(base_name)],
        outs = ["out/{}.bzl".format(base_name)],
        # tags = ["manual"],
    )

    filename = get_build_filename(host, version)
    update_ws_file(
        name = base_name,
        origin = "{}/deps".format(base_name),
        target = filename,
    )
