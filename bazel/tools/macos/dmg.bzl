"""Run pkgbuild to create a Macos PKG"""

def _create_dmg_impl(ctx):
    app_dir = ctx.actions.declare_directory(ctx.attr.app_name)
    # contents_dir = ctx.actions.declare_directory(ctx.attr.app_name + "/Contents")

    # We extract the payload and run `pkgbuild` in the same command so it uses
    # actual files and not the symlinked ones created by Bazel
    output_pkg = ctx.outputs.pkg
    args = ctx.actions.args()
    args.add(ctx.file.contents)
    args.add(app_dir.path + "/Contents")
    args.add(output_pkg)
    args.add(app_dir.path)
    args.add(app_dir.dirname)
    ctx.actions.run_shell(
        inputs = [ctx.file.contents],
        outputs = [output_pkg, app_dir],
        arguments = [args],
        command = """
            mkdir -p $2
            tar -xzf $1 -C $2
            tree -a .

            codesign --force --deep --sign - $4
            # echo ">>>>> $5"
            mkdir -p $5/tmp
            cp -R $4 $5/tmp/
            hdiutil create -verbose -fs APFS -volname "FinancesApp" -srcfolder $5/tmp $3
        """.format(app_name = ctx.attr.app_name),
    )

create_dmg = rule(
    implementation = _create_dmg_impl,
    attrs = {
        "app_name": attr.string(mandatory = True),
        "contents": attr.label(mandatory = True, allow_single_file = True),
    },
    outputs = {"pkg": "%{app_name}.dmg"},
)
