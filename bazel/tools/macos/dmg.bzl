"""Run pkgbuild to create a Macos PKG"""

def _create_app_impl(ctx):
    contents_dir = ctx.actions.declare_directory(ctx.attr.app_name + "/Contents")

    # We extract the payload and run `pkgbuild` in the same command so it uses
    # actual files and not the symlinked ones created by Bazel
    output_pkg = ctx.outputs.pkg
    args = ctx.actions.args()
    args.add(ctx.file.contents)
    args.add(contents_dir.path)
    args.add(output_pkg)
    ctx.actions.run_shell(
        inputs = [ctx.file.contents],
        outputs = [output_pkg, contents_dir],
        arguments = [args],
        command = """
            tar -xzf $1 -C $2
            tree -a .

            codesign --force --deep --sign - {}
            echo "Done" >> $3
        """.format(ctx.attr.app_name),
    )

create_app = rule(
    implementation = _create_app_impl,
    attrs = {
        "app_name": attr.string(mandatory = True),
        "contents": attr.label(mandatory = True, allow_single_file = True),
    },
    outputs = {"pkg": "%{name}.dmg"},
)
