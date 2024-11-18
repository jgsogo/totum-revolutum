"""A rule that ensures a docker container is running while executing another binary"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

#FIXME: HAve a look to https://github.com/bazel-contrib/rules_oci/blob/main/oci/private/load.bzl

def _with_docker_compose_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)
    # for out in ctx.outputs.outs:
    #     print("Declare output: {}".format(out))
    #     ctx.actions.declare_file(out.path)

    # Compose files
    compose_files = [it.path for it in ctx.files.docker_compose]

    # Write down the environment file
    environment_file = ctx.actions.declare_file(ctx.label.name + ".env")
    content = [
        "COMPOSE_FILE={}".format(":".join(compose_files)),
        "COMPOSE_PATH_SEPARATOR=:",
    ]
    for k, v in ctx.attr.env.items():
        content.append("{}={}".format(k, v))
    ctx.actions.write(
        output = environment_file,
        content = "\n".join(content),
    )

    # Expand the 'cmd' we are going to run
    cmd = ctx.expand_location(ctx.attr.cmd, targets = ctx.attr.srcs + ctx.attr.tools)
    print("cmd: {}".format(cmd))

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "{{BASH_RLOCATION_FUNCTION}}": BASH_RLOCATION_FUNCTION,
            "{{docker_cli}}": to_rlocation_path(ctx, ctx.file.docker_cli) if ctx.file.docker_cli else "",
            "%ENV_FILE%": environment_file.path,
            "%PROJECT_NAME%": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
            "%SERVICES%": " ".join(ctx.attr.docker_services),
            # "%BINARY%": ctx.file.binary.short_path,
            "%CMD%": cmd,
        },
        is_executable = True,
    )

    print(">>>>>>")

    # print(ctx.attr.binary[DefaultInfo].data_runfiles)
    print(">>>>>>")

    # output_provider = ctx.attr.binary[OutputGroupInfo]
    runtime_deps = []
    if ctx.file.docker_cli:
        runtime_deps.append(ctx.file.docker_cli)

    # all_inputs = ctx.attr.binary[DefaultInfo].default_runfiles.files
    print(ctx.attr._runfiles[DefaultInfo].default_runfiles.files.to_list())
    ctx.actions.run(
        inputs = ctx.attr._runfiles[DefaultInfo].default_runfiles.files.to_list() + runtime_deps + [environment_file] + ctx.files.docker_compose + ctx.files.srcs + ctx.files.tools,
        outputs = ctx.outputs.outs,
        executable = executable,
        # tools = [ctx.executable._dbgen],
        # arguments = [args],
        # mnemonic = "Execute binary in the context of the docker-compose",
        env = ctx.attr.env,
    )

    # ctx.actions.run_shell(
    #     inputs = [environment_file] + ctx.files.docker_compose + ctx.files.srcs + ctx.files.tools,
    #     outputs = ctx.outputs.outs,
    #     executable = executable,
    #     # tools = [ctx.executable._dbgen],
    #     # arguments = [args],
    #     # mnemonic = "Execute binary in the context of the docker-compose",
    #     env = ctx.attr.env
    # )
    # ctx.actions.run_shell(
    #     outputs = ctx.attr.binary[DefaultInfo].files.to_list(),
    #     inputs = [executable_script, ],
    #     # arguments = [ctx.executable.binary.path, ctx.outputs.executable.path],
    #     command = "{}".format(executable_script.short_path),
    # )

    runfiles = ctx.runfiles(files = runtime_deps + [environment_file] + ctx.files.docker_compose + ctx.files.srcs + ctx.files.tools)

    # for dep in ctx.attr.deps:
    #     runfiles = runfiles.merge(dep[DefaultInfo].data_runfiles)
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    return [
        DefaultInfo(
            # executable = executable,
            runfiles = runfiles,
        ),
        # RunEnvironmentInfo(
        #     environment = ctx.attr.env,
        # ),
        # OutputGroupInfo(
        #     # debug_files = depset([debug_file]),
        #     all_files = ctx.outputs.outs,
        # ),
    ]

with_docker_compose = rule(
    implementation = _with_docker_compose_impl,
    attrs = {
        "docker_compose": attr.label_list(
            doc = "Docker compose file to run",
            allow_files = True,
            mandatory = True,
        ),
        "docker_services": attr.string_list(
            doc = "Comma-separated list with the services to start",
            default = [],
        ),
        "cmd": attr.string(
            mandatory = True,
        ),
        "tools": attr.label_list(
            allow_files = True,
        ),
        "srcs": attr.label_list(
            allow_files = True,
        ),
        # "binary": attr.label(
        #     doc = "Binary to run",
        #     mandatory = True,
        #     # executable = True,
        #     # cfg = "exec",
        #     allow_single_file = True,
        # ),
        "outs": attr.output_list(
            doc = "Output of the binary",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_docker_binary.tpl.sh"),
            allow_single_file = True,
        ),
        # "data": attr.label_list(
        #     doc = "Additional data",
        # ),
        "env": attr.string_dict(
            doc = "Environment variables",
        ),
        "deps": attr.label_list(
            doc = "Additional dependencies",
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
        "docker_cli": attr.label(
            doc = """\
                Alternative target for a container cli tool that will be
                used to load the image into the local engine when using `bazel run` on this target.

                By default, we look for `docker` or `podman` on the PATH, and run the `load` command.

                See the _run_template attribute for the script that calls this loader tool.
                """,
            allow_single_file = True,
            mandatory = False,
            executable = True,
            cfg = "target",
        ),
    },
    # executable = True,
    doc = """Ensure docker 'image' is running while executing the 'binary'""",
)
