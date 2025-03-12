"""A rule that ensures that Postgres (in a docker container) is running while executing another binary"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

POSTGRES_IMAGE_TAG = "17"

def _with_envsubst_run_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Render the script we are executing
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%ENVSUBST%": to_rlocation_path(ctx, ctx.file._envsubst),
            "%CONFIG%": to_rlocation_path(ctx, ctx.file.config),
            "%BINARY%": to_rlocation_path(ctx, ctx.file.binary),
            # "%CONTAINER_NAME%": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
            # "%POSTGRES_IMAGE_TAG%": ctx.attr.postgres_image_tag,
            # "%ENV_FILE%": env_file.short_path,
            # "%BINARIES%": " ".join(binaries),
            # "%ENV_TRANSPOSE%": "\n".join(env_transposition),  # TODO: Create another file and source it here
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles([ctx.file._envsubst, ctx.file.config])
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.binary.default_runfiles)

    # Environment
    environment = ctx.attr.env
    if RunEnvironmentInfo in ctx.attr.binary:
        environment = environment | ctx.attr.binary[RunEnvironmentInfo].environment

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
        RunEnvironmentInfo(
            environment = ctx.attr.env,
        ),
    ]

with_envsubst_run = rule(
    implementation = _with_envsubst_run_impl,
    attrs = {
        "config": attr.label(
            allow_single_file = True,
            doc = "Configuration files to use",
        ),
        "binary": attr.label(
            doc = "Binary to execute",
            allow_single_file = True,
            mandatory = True,
            cfg = "target",
        ),
        "env": attr.string_dict(
            doc = "Environment variables",
        ),
        "_envsubst": attr.label(
            doc = "Envsubst application",
            allow_single_file = True,
            cfg = "exec",
            default = "//tools/envsubst",
        ),
        "_run_template": attr.label(
            default = Label("//tools/envsubst:with_envsubst_run.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    executable = True,
    doc = """Execute envsubst before running the binary and pass the resulting configuration file to the binary""",
)
