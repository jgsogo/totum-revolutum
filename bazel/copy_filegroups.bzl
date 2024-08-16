"""Provides 'copy_filegroups' rule to copy files from the source folder into the build workspace"""

def _copy_filegroup_impl(ctx):
    all_input_files = [
        f
        for t in ctx.attr.targeted_filegroups
        for f in t.files.to_list()
    ]

    all_outputs = []
    for f in all_input_files:
        if ctx.attr.flatten:
            out_path = "{}/{}".format(ctx.attr.folder, f.basename)
        else:
            out_path = "{}/{}".format(ctx.attr.folder, f.short_path.removeprefix(ctx.attr.strip_prefix))
        out = ctx.actions.declare_file(out_path)
        all_outputs.append(out)

    for f, out in zip(all_input_files, all_outputs):
        ctx.actions.run_shell(
            outputs = [out],
            inputs = depset([f]),
            arguments = [f.path, out.path],
            command = "cp $1 $2",
        )

    # Small sanity check
    if len(all_input_files) != len(all_outputs):
        fail("Output count should be 1-to-1 with input count.")

    return [
        DefaultInfo(
            files = depset(all_outputs),
            runfiles = ctx.runfiles(files = all_outputs),
        ),
    ]

copy_filegroups = rule(
    implementation = _copy_filegroup_impl,
    attrs = {
        "targeted_filegroups": attr.label_list(allow_files = True),
        "folder": attr.string(
            default = ".",
            doc = "Target folder to copy files to (inside this package)",
            mandatory = False,
        ),
        "flatten": attr.bool(
            default = False,
            doc = "If the directory structure should be preserved or not,",
        ),
        "strip_prefix": attr.string(
            default = "",
            doc = "Common path prefix to all files to remove in the target directory.",
        ),
    },
)
