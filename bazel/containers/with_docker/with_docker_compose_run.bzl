"""A rule that ensures a docker container is running while executing another binary"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

#FIXME: HAve a look to https://github.com/bazel-contrib/rules_oci/blob/main/oci/private/load.bzl

def _with_docker_compose_run_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Environment: we will use both a file (for docker) and environment variables
    env = {
        "SQL_USER": "with_docker_compose",
        # "POSTGRES_DB": "with_postgres",
        "SQL_PASSWORD": "with_docker_compose",
    } | ctx.attr.env

    env_file = ctx.actions.declare_file(ctx.label.name + ".env")
    env_file_content = []
    for key, value in env.items():
        env_file_content.append("{}={}".format(key, value))
    ctx.actions.write(
        output = env_file,
        content = "\n".join(env_file_content),
    )

    binaries = []
    for dep in ctx.attr.binaries:
        binaries.append(to_rlocation_path(ctx, dep.files_to_run.executable))

    env_transposition = []
    for key, value in ctx.attr.env_transpose.items():
        env_transposition.append("export {}=${}".format(key, value))

    docker_compose_files = []
    for dc_file in ctx.files.docker_compose:
        docker_compose_files.append("--file=$(pwd)/{}".format(dc_file.short_path))

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%DOCKER_CLI%": to_rlocation_path(ctx, ctx.file.docker_cli) if ctx.file.docker_cli else "",
            "%PROJECT_NAME%": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
            "%SERVICES%": " ".join(ctx.attr.docker_services),
            "%ENV_FILE%": env_file.short_path,
            "%BINARIES%": " ".join(binaries),
            "%ENV_TRANSPOSE%": "\n".join(env_transposition),  # TODO: Create another file and source it here
            "%DOCKER_COMPOSE_FILES%": " ".join(docker_compose_files),
        },
        is_executable = True,
    )

    runtime_deps = ctx.files.docker_compose + [env_file]
    if ctx.file.docker_cli:
        runtime_deps.append(ctx.file.docker_cli)

    runfiles = ctx.runfiles(runtime_deps)
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    for dep in ctx.attr.binaries:
        runfiles = runfiles.merge(dep.default_runfiles)

    runfiles = runfiles.merge_all([
        dep[DefaultInfo].default_runfiles
        for dep in ctx.attr.binaries
    ])

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
        RunEnvironmentInfo(
            environment = env,
        ),
    ]

with_docker_compose_run = rule(
    implementation = _with_docker_compose_run_impl,
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
        "binaries": attr.label_list(
            doc = "Binaries to execute while the container is running",
            mandatory = True,
            cfg = "target",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_docker_compose_run.tpl.sh"),
            allow_single_file = True,
        ),
        "env": attr.string_dict(
            doc = "Environment variables",
        ),
        "env_transpose": attr.string_dict(
            doc = "Environment variables that will be populated with the value of others",
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

with_docker_compose_test = rule(
    implementation = _with_docker_compose_run_impl,
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
        "binaries": attr.label_list(
            doc = "Binaries to execute while the container is running",
            mandatory = True,
            cfg = "target",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_docker_compose_run.tpl.sh"),
            allow_single_file = True,
        ),
        "env": attr.string_dict(
            doc = "Environment variables",
        ),
        "env_transpose": attr.string_dict(
            doc = "Environment variables that will be populated with the value of others",
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
    test = True,
    doc = """Ensure docker compose services are running while executing the 'binary'""",
)
