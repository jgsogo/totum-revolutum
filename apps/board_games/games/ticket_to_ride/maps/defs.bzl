"""Generate SVG files for the maps
"""

load("@bazel_skylib//rules:diff_test.bzl", "diff_test")
load("@rules_shell//shell:sh_test.bzl", "sh_test")
load("//bazel:run_copy_to_workspace.bzl", "run_copy_to_workspace")

def generate_map_targets(name):
    """Creates all the targets for a given TextProto file

    The main target that it generates is a filegroup with all the resources needed to operate
    with this map:
     * the `<name>.textproto` file, with the data
     * the `<name>.svg` file, with the main SVG
     * the `<name>-background.svg` file, with the background image used by the main SVG

    Args:
        name: the name of the map. It has to match an input `<name>.textproto` file
    """

    native.filegroup(
        name = name,
        srcs = [
            "{}.textproto".format(name),
            "{}.svg".format(name),
            "{}-background.svg".format(name),
        ],
        visibility = [
            "//apps/board_games/games/ticket_to_ride:__subpackages__",
            "//apps/board_games/webapp/src/lib/ticket_to_ride:__pkg__",
            "//apps/board_games/webapp/static/ticket_to_ride:__pkg__",
        ],
    )

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

    native.genrule(
        name = "{}.test.map.build".format(name),
        outs = ["{}.test.map.build.sh".format(name)],
        cmd = """
            echo '#!/bin/bash' > $@
            echo '$(rootpath //apps/board_games/games/ticket_to_ride/maps/tests:test_map) \\' >> $@
            echo '  --textproto=$(location :{name}-input) \' >> $@
        """.format(name = name),
        tools = [
            "//apps/board_games/games/ticket_to_ride/maps/tests:test_map",
        ],
        srcs = [
            ":{}-input".format(name),
        ],
        visibility = ["//visibility:private"],
        testonly = True,
        executable = True,
    )

    sh_test(
        name = "{}.test.map".format(name),
        data = [":{}-input".format(name), "//apps/board_games/games/ticket_to_ride/maps/tests:test_map"],
        srcs = [":{}.test.map.build".format(name)],
    )
