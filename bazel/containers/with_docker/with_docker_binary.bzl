"""A rule that ensures a docker container is running while executing another binary"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

#FIXME: HAve a look to https://github.com/bazel-contrib/rules_oci/blob/main/oci/private/load.bzl

def _with_docker_compose_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Compose environment
    compose_files = [it.short_path for it in ctx.files.docker_compose]
    environment = {
        "COMPOSE_PROJECT_NAME": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
        "COMPOSE_FILE": ":".join(compose_files),
        "COMPOSE_PATH_SEPARATOR": ":",
    }

    # Expand the 'cmd' we are going to run
    cmd = ctx.expand_location(ctx.attr.cmd, targets = ctx.attr.tools)

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%DOCKER_CLI%": to_rlocation_path(ctx, ctx.file.docker_cli) if ctx.file.docker_cli else "",
            "%SERVICES%": " ".join(ctx.attr.docker_services),
            "%CMD%": cmd,
        },
        is_executable = True,
    )

    runtime_deps = ctx.files.docker_compose + ctx.files.tools
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
            environment = ctx.attr.env | environment,
        ),
    ]

with_docker_compose = rule(
    implementation = _with_docker_compose_impl,
    attrs = {
        "docker_compose": attr.label_list(
            doc = "Docker compose file/s to run",
            allow_files = True,
            mandatory = True,
        ),
        "docker_services": attr.string_list(
            doc = "Service/s to start",
            default = [],
        ),
        "cmd": attr.string(
            mandatory = True,
        ),
        "tools": attr.label_list(
            allow_files = True,
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_docker_binary.tpl.sh"),
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
    doc = """Ensure docker compose services are running while executing the 'binary'""",
)
