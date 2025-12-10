"""Generate SVG files for the maps
"""

load("@bazel_skylib//rules:diff_test.bzl", "diff_test")
load("//bazel:run_copy_to_workspace.bzl", "run_copy_to_workspace")

def generate_map_targets(name):
    native.filegroup(
        name = "{}-input".format(name),
        srcs = ["{}.textproto".format(name)],
        visibility = ["//apps/board_games/games/ticket_to_ride:__subpackages__"],
    )

    native.genrule(
        name = "{}-svg".format(name),
        srcs = [":{}-input".format(name)],
        outs = ["{}.out.svg".format(name)],
        cmd = "$(location //apps/board_games/games/ticket_to_ride/maps/gen) --textproto=$(location :{}-input) --output=$@".format(name),
        tools = [
            "//apps/board_games/games/ticket_to_ride/maps/gen",
        ],
    )

    run_copy_to_workspace(
        name = "{}.update".format(name),
        origin = ":{}-svg".format(name),
        tags = ["update", "manual"],
        target = "{}.svg".format(name),
    )

    diff_test(
        name = "{}.update.test".format(name),
        file1 = ":{}-svg".format(name),
        file2 = "{}.svg".format(name),
        tags = ["check"],
        failure_message = "\n\nRun `bazel run {}:{}.update` to update the schema file".format(native.package_name(), name),
    )
