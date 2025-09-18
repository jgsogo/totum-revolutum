"""
A function to create the BUILD file for a repo
"""

load("@rules_qt//:constants.bzl", "ARCHS", "HOSTS", "OUTPUT_DIR_FOR_HOST", "TARGET_SDKS")
load("@rules_qt//private:utils.bzl", "get_base_name", "get_build_filename")
load("@rules_qt//tools:update_ws_file.bzl", "update_ws_file")

def create_defs_file_and_update_test(name, host, target_sdk, version, arch):
    """
    Generates the BUILD file for a repo

    Args:
        name:
        host:
        target_sdk:
        version:
        arch:
    """

    base_name = get_base_name(name, version, host, arch, target_sdk)
    output_filename = get_build_filename(name, version, host, arch, target_sdk)

    create_defs_file(
        name = base_name,
        arch = arch,
        host = host,
        target_sdk = target_sdk,
        version = version,
    )

    update_ws_file(
        name = base_name,
        origin = ":{}".format(base_name),
        target = ":{}".format(output_filename),
    )

def _create_defs_file(ctx):
    # Execute AQT to install Qt
    install_folder = ctx.actions.declare_directory(ctx.attr.name + ".install")

    args = ctx.actions.args()
    args.add("install-qt")
    args.add(ctx.attr.host)
    args.add(ctx.attr.target_sdk)
    args.add(ctx.attr.version)
    args.add(ctx.attr.arch)
    args.add("--outputdir")
    args.add(install_folder.path)

    ctx.actions.run(
        outputs = [install_folder],
        arguments = [args],
        progress_message = "Installing Qt {} for {}".format(ctx.attr.version, ctx.attr.host),
        executable = ctx.executable._aqt,
    )

    # Execute our tool to generate the BUILD file
    output = ctx.actions.declare_file(ctx.attr.name + ".out")

    args = ctx.actions.args()
    args.add("--input_path")
    args.add(install_folder.path + "/{}/{}".format(ctx.attr.version, OUTPUT_DIR_FOR_HOST.get(ctx.attr.host, ctx.attr.host)))
    args.add("--base_path")
    args.add(install_folder.path)
    args.add("--output")
    args.add(output)
    args.add("--host")
    args.add(ctx.attr.host)
    args.add("--target_sdk")
    args.add(ctx.attr.target_sdk)
    args.add("--version")
    args.add(ctx.attr.version)
    args.add("--arch")
    args.add(ctx.attr.arch)

    ctx.actions.run(
        inputs = [install_folder],
        outputs = [output],
        arguments = [args],
        progress_message = "Creating BUILD file",
        executable = ctx.executable._repo_build_tool,
    )

    # We could generate a `.tar.gz` too and store it somewhere. This way we will be sure that the
    # generated BUILD file matches the repo

    return DefaultInfo(files = depset([output]))

create_defs_file = rule(
    implementation = _create_defs_file,
    attrs = {
        "host": attr.string(
            mandatory = True,
            values = HOSTS,
        ),
        "target_sdk": attr.string(
            mandatory = True,
            values = TARGET_SDKS,
        ),
        "arch": attr.string(
            mandatory = True,
            values = ARCHS,
        ),
        "version": attr.string(
            mandatory = True,
        ),
        "_aqt": attr.label(
            executable = True,
            default = Label("@rules_qt//tools/aqt"),
            cfg = "exec",
        ),
        "_repo_build_tool": attr.label(
            executable = True,
            default = Label("@rules_qt//tools/deps"),
            cfg = "exec",
        ),
    },
)
