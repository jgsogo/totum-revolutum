"""
"""

load("@bazel_skylib//rules:diff_test.bzl", "diff_test")
load("@rules_shell//shell:sh_binary.bzl", "sh_binary")

def update_ws_file(name, origin, target):
    """
    Checks and copy file to workspace

    Args:
        name:
        origin:
        target:
    """

    failure_message = """\n\n
        Update workspace file using:

        $> bazel run //{}:{}.update
    """.format(native.package_name(), name)

    diff_test(
        name = "{}.test".format(name),
        failure_message = failure_message,
        file1 = origin,
        file2 = target,
    )

    sh_binary(
        name = "{}.update".format(name),
        srcs = ["//tools:update_ws_file.sh"],
        data = [origin, target],
        args = [
            "$(location {})".format(origin),
            "$(location {})".format(target),
        ],
        tags = ["manual"],
    )
