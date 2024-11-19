"""A rule that ensures a docker container is running while executing another binary"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

#FIXME: HAve a look to https://github.com/bazel-contrib/rules_oci/blob/main/oci/private/load.bzl

def _with_docker_run_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    env = {
        "POSTGRES_USER": "with_postgres",
        "POSTGRES_DB": "with_postgres",
        "POSTGRES_PASSWORD": "with_postgres",
    } | ctx.attr.env

    env_file = ctx.actions.declare_file(ctx.label.name + ".env")
    env_file_content = []
    for key, value in env.items():
        env_file_content.append("{}={}".format(key, value))
    ctx.actions.write(
        output = env_file,
        content = "\n".join(env_file_content),
    )

    # Expand the 'cmd' we are going to run
    binaries = []
    for dep in ctx.attr.binaries:
        binaries.append(to_rlocation_path(ctx, dep.files_to_run.executable))

    # cmd = ctx.expand_location(ctx.attr.cmd, targets = ctx.attr.tools)

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%DOCKER_CLI%": to_rlocation_path(ctx, ctx.file.docker_cli) if ctx.file.docker_cli else "",
            "%CONTAINER_NAME%": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
            "%POSTGRES_IMAGE_TAG%": ctx.attr.postgres_image_tag,
            "%LIVENESS_PROBE%": "liveness-probe",
            # "%CMD%": cmd,
            "%ENV_FILE%": env_file.short_path,
            "%BINARIES%": " ".join(binaries),
        },
        is_executable = True,
    )

    runtime_deps = [env_file]
    if ctx.file.docker_cli:
        runtime_deps.append(ctx.file.docker_cli)

    binaries_files = []
    for dep in ctx.files.binaries:
        binaries_files.append(dep)

    runfiles = ctx.runfiles(runtime_deps, transitive_files = depset(binaries_files))
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    # for tool in ctx.attr.tools:
    for dep in ctx.attr.binaries:
        runfiles = runfiles.merge(dep.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
        RunEnvironmentInfo(
            environment = env,
        ),
    ]

with_postgres_run = rule(
    implementation = _with_docker_run_impl,
    attrs = {
        "postgres_image_tag": attr.string(
            doc = "Docker image to run",
        ),
        # "cmd": attr.string(
        #     mandatory = True,
        # ),
        # "tools": attr.label_list(
        #     allow_files = True,
        # ),
        "binaries": attr.label_list(
            doc = "Binaries to execute while the container is running",
            mandatory = True,
            cfg = "target",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_postgres_run.tpl.sh"),
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
    doc = """Ensure postgres is running while executing the 'binary'. The binary will receive the postgres URL as argument""",
)
