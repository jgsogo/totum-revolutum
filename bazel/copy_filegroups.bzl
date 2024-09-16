"""Provides a rule to copy filegroups to a given path

Copied from https://github.com/pestophagous/examples/blob/master/cpp-tutorial/stage1/main/copy_filegroups.bzl,
here I'm just adding 'folder' additional argument and fixing minor things
"""

def _copy_filegroup_impl(ctx):
    all_input_files = [
        f
        for t in ctx.attr.targeted_filegroups
        for f in t.files.to_list()
    ]

    all_outputs = []
    for f in all_input_files:
        short_path = f.short_path
        if ctx.attr.strip_prefix:
            short_path = short_path.removeprefix(ctx.attr.strip_prefix)
        out_path = "{}/{}".format(ctx.attr.folder, short_path)
        out = ctx.actions.declare_file(out_path)
        all_outputs.append(out)
        ctx.actions.run_shell(
            outputs = [out],
            inputs = depset([f]),
            arguments = [f.path, out.path],
            # This is what we're all about here. Just a simple 'cp' command.
            # Copy the input to CWD/f.basename, where CWD is the package where
            # the copy_filegroups_to_this_package rule is invoked.
            # (To be clear, the files aren't copied right to where your BUILD
            # file sits in source control. They are copied to the 'shadow tree'
            # parallel location under `bazel info bazel-bin`)
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
        "folder": attr.string(
            default = ".",
            doc = "Target folder to copy files to (inside this package)",
            mandatory = False,
        ),
        "strip_prefix": attr.string(
            doc = "Strip this prefix from all the paths",
            mandatory = False,
        ),
        "targeted_filegroups": attr.label_list(),
    },
)
