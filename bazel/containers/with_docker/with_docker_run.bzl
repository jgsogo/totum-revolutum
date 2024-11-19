"""A rule that ensures a docker container is running while executing another binary"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

#FIXME: HAve a look to https://github.com/bazel-contrib/rules_oci/blob/main/oci/private/load.bzl

def _with_docker_run_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    env_file = ctx.actions.declare_file(ctx.label.name + ".env")
    env_file_content = []
    for key, value in ctx.attr.env.items():
        env_file_content.append("{}={}".format(key, value))
    ctx.actions.write(
        output = env_file,
        content = "\n".join(env_file_content),
    )

    # Expand the 'cmd' we are going to run
    # cmd = ctx.expand_location(ctx.attr.cmd, targets = ctx.attr.tools)

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%DOCKER_CLI%": to_rlocation_path(ctx, ctx.file.docker_cli) if ctx.file.docker_cli else "",
            "%CONTAINER_NAME%": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
            "%INNER_PORT%": "5432",
            "%IMAGE%": ctx.attr.image,
            "%LIVENESS_PROBE%": "liveness-probe",
            "%CMD%": "cmd",
            "%ENV_FILE%": env_file.short_path,
        },
        is_executable = True,
    )

    runtime_deps = ctx.files.tools + [env_file]
    if ctx.file.docker_cli:
        runtime_deps.append(ctx.file.docker_cli)

    runfiles = ctx.runfiles(runtime_deps)
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
        RunEnvironmentInfo(
            environment = ctx.attr.env,
        ),
    ]

with_docker_run = rule(
    implementation = _with_docker_run_impl,
    attrs = {
        "image": attr.string(
            doc = "Docker image to run",
        ),
        "cmd": attr.string(
            mandatory = True,
        ),
        "tools": attr.label_list(
            allow_files = True,
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_docker_run.tpl.sh"),
            allow_single_file = True,
        ),
        "env": attr.string_dict(
            doc = "Environment variables",
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
        "docker_cli": attr.label(
            doc = """\
                Alternative target for a container cli tool that will be
                used to run docker compose when using `bazel run` on this target.

                By default, we look for `docker` or `podman` on the PATH.

                See the _run_template attribute for the script that calls this docker tool.
                """,
            allow_single_file = True,
            mandatory = False,
            executable = True,
            cfg = "target",
        ),
    },
    executable = True,
    doc = """Ensure docker container is running while executing the 'binary'""",
)
