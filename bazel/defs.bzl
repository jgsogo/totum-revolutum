"""Copy a file to the workspace
"""

def _run_copy_to_workspace_impl(ctx):
    ctx.actions.write(
        output = ctx.outputs.executable,
        content = "cd $BUILD_WORKSPACE_DIRECTORY && cp -fv {} {}".format(ctx.file.origin.path, ctx.file.target.path),
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = [ctx.file.origin])

    return [
        DefaultInfo(
            executable = ctx.outputs.executable,
            runfiles = runfiles,
        ),
    ]

run_copy_to_workspace = rule(
    implementation = _run_copy_to_workspace_impl,
    attrs = {
        "origin": attr.label(
            allow_single_file = True,
            doc = "File with the new benchmark results",
            mandatory = True,
        ),
        "target": attr.label(
            doc = "File to be overriden in the workspace",
            mandatory = True,
            allow_single_file = True,
        ),
    },
    executable = True,
    doc = """Overrides a file in the user workspace

    Use this RUN rule to update files in the workspace with content
    created by other rules.
    """,
)
