"""Provides a rule to extract a Rust crate from a workspace"""

def _extract_from_workspace_impl(ctx):
    """The implementation of the `extract_from_workspace` rule

    Args:
        ctx (ctx): The rule's context object
    """

    args = ctx.actions.args()
    args.add("--workspace")
    args.add(ctx.file.workspace)
    args.add("--package")
    args.add(ctx.file.package)
    args.add("--output")
    args.add(ctx.outputs.output_cargo_toml)

    ctx.actions.run(
        mnemonic = "WorkspaceSplit",
        progress_message = "Extract package from workspace",
        outputs = [ctx.outputs.output_cargo_toml],
        executable = ctx.executable._extract_package,
        inputs = [ctx.file.workspace, ctx.file.package],
        arguments = [args],
    )

extract_from_workspace = rule(
    implementation = _extract_from_workspace_impl,
    attrs = {
        "output_cargo_toml": attr.output(
            doc = "Path to the generated Cargo.toml file",
            mandatory = False,
        ),
        "workspace": attr.label(
            doc = "Path to the workspace Cargo.toml file",
            allow_single_file = True,
        ),
        "package": attr.label(
            allow_single_file = True,
            doc = "Path to the package Cargo.toml file",
        ),
        "_extract_package": attr.label(
            doc = "A tool to extract a package from a workspace",
            default = Label("@totum//bazel/rust/ws:extract_package"),
            cfg = "exec",
            executable = True,
        ),
    },
    doc = "Extracts a Rust crate from a workspace generating an isolated Cargo.toml file",
)
