"""A rule that ensures a docker container is running while executing another binary"""

def _with_docker_compose_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Write down the environment file
    environment_file = ctx.actions.declare_file(ctx.label.name + ".env")
    content = ["SECRET_KEY=this-should-not-be-used-in-production"]
    ctx.actions.write(
        output = environment_file,
        content = "\n".join(content),
    )

    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%COMPOSE_FILE%": ctx.file.docker_compose.short_path,
            "%ENV_FILE%": environment_file.short_path,
            "%PROJECT_NAME%": ctx.label.package.replace("/", "_") + "_" + ctx.label.name,
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = [environment_file, ctx.file.docker_compose])

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

with_docker_compose = rule(
    implementation = _with_docker_compose_impl,
    attrs = {
        "docker_compose": attr.label(
            doc = "Docker compose file to run",
            allow_single_file = True,
            mandatory = True,
        ),
        "binary": attr.label(
            doc = "Binary to run",
            mandatory = True,
            executable = True,
            cfg = "exec",
            allow_single_file = True,
        ),
        "_run_template": attr.label(
            default = Label("//bazel/containers/with_docker:with_docker_binary.tpl.sh"),
            allow_single_file = True,
        ),
        # "data": attr.label_list(
        #     doc = "Additional data",
        # ),
        # "env": attr.string_dict(
        #     doc = "Environment variables",
        # ),
    },
    executable = True,
    doc = """Ensure docker 'image' is running while executing the 'binary'""",
)
