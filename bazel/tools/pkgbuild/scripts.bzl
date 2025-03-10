"""Generate preinstall script for pkgbuild"""

def _composable_script_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.output)

    pre_files = []

    # Prelude
    prelude = ctx.actions.declare_file(ctx.label.name + "-prelude")
    ctx.actions.expand_template(
        template = ctx.file.prelude,
        output = prelude,
        substitutions = {
            "%NAME%": ctx.attr.pkgbuild_name,
            "%LOG_LEVEL%": ctx.attr.log_level,
        },
    )
    pre_files.append(prelude)

    # The logger
    pre_files.append(ctx.file._logger)

    # Configure the logger + Some variables with some folders
    folders = ctx.actions.declare_file(ctx.label.name + "-folders")
    content = ["""
log_with_timestamp $LOGFILE

# Provide folders as environment variables
export INSTALL_FOLDER="$3"
log_debug "INSTALL_FOLDER: $INSTALL_FOLDER"
"""]
    for key, folder in ctx.attr.folders.items():
        content.append("""
{key}="${{INSTALL_FOLDER}}{value}"
export {key}="$(realpath ${key} || ${key})"
log_debug "{key}: '${{INSTALL_FOLDER}}{value}' resolved to '${key}'"
""".format(key = key, value = folder))
    ctx.actions.write(
        output = folders,
        content = "\n".join(content),
    )
    pre_files.append(folders)

    # Ending
    ending = ctx.actions.declare_file(ctx.label.name + "-ending")
    ctx.actions.expand_template(
        template = ctx.file.ending,
        output = ending,
        substitutions = {
            "%NAME%": ctx.attr.pkgbuild_name,
        },
    )

    chunks_files = [f.path for f in pre_files] + [f.path for f in ctx.files.chunks] + [ending.path]

    ctx.actions.run_shell(
        inputs = ctx.files.chunks + pre_files + [ending],
        outputs = [output],
        command = "cat {files} > {output}".format(files = " ".join(chunks_files), output = output.path),
    )

    return [
        DefaultInfo(files = depset([output])),
    ]

_composable_script = rule(
    implementation = _composable_script_impl,
    attrs = {
        "pkgbuild_name": attr.string(mandatory = True),
        "chunks": attr.label_list(
            mandatory = True,
            allow_files = True,
        ),
        "prelude": attr.label(
            mandatory = True,
            allow_single_file = True,
        ),
        "ending": attr.label(
            mandatory = True,
            allow_single_file = True,
        ),
        "output": attr.string(),
        "folders": attr.string_dict(),
        # "bin_folder": attr.string(),
        # "logs_folder": attr.string(),
        # "run_folder": attr.string(),
        "_logger": attr.label(
            default = Label("//bazel/tools/pkgbuild/scripts/bash:logger.sh"),
            allow_single_file = True,
        ),
        "log_level": attr.string(
            default = "DEBUG",
        ),
    },
    doc = "A rule to create a script with a prelude, some chunks and an ending",
)

def preinstall(name, pkgbuild_name, chunks, **kwargs):
    _composable_script(
        name = name,
        pkgbuild_name = pkgbuild_name,
        chunks = chunks,
        prelude = "//bazel/tools/pkgbuild:prelude.sh",
        ending = "//bazel/tools/pkgbuild:empty.sh",
        output = "preinstall",
        **kwargs
    )

def postinstall(name, pkgbuild_name, chunks, **kwargs):
    _composable_script(
        name = name,
        pkgbuild_name = pkgbuild_name,
        chunks = chunks,
        prelude = "//bazel/tools/pkgbuild:prelude.sh",
        ending = "//bazel/tools/pkgbuild:empty.sh",
        output = "postinstall",
        **kwargs
    )
