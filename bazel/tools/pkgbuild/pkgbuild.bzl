"""Run pkgbuild to create a Macos PKG"""

def _pkgbuild_impl(ctx):
    payload_dir = ctx.actions.declare_directory(ctx.label.name + "-payload")
    scripts_dir = ctx.actions.declare_directory(ctx.label.name + "-scripts")

    # We extract the payload and run `pkgbuild` in the same command so it uses
    # actual files and not the symlinked ones created by Bazel
    output_pkg = ctx.outputs.pkg
    args = ctx.actions.args()
    args.add(ctx.file.root)
    args.add(payload_dir.path)
    args.add(ctx.attr.identifier)
    args.add(ctx.attr.version)
    args.add(output_pkg)
    args.add(ctx.file.scripts)
    args.add(scripts_dir.path)
    ctx.actions.run_shell(
        inputs = [ctx.file.root, ctx.file.scripts],
        outputs = [output_pkg, payload_dir, scripts_dir],
        arguments = [args],
        command = """
            tar -xzf $1 -C $2
            tar -xzf $6 -C $7
            pkgbuild --root $2 --scripts $7 --identifier $3 --version $4 $5
        """,
    )

pkgbuild = rule(
    implementation = _pkgbuild_impl,
    attrs = {
        "identifier": attr.string(mandatory = True),
        "version": attr.string(mandatory = True),
        "root": attr.label(mandatory = True, allow_single_file = True),
        "scripts": attr.label(allow_single_file = True),
    },
    outputs = {"pkg": "%{name}.pkg"},
)
