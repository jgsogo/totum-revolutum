"""Generate preinstall script for pkgbuild"""

def _composable_script_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.output)

    prelude = ctx.actions.declare_file(ctx.label.name + "-prelude")
    ctx.actions.expand_template(
        template = ctx.file.prelude,
        output = prelude,
        substitutions = {
            "%NAME%": ctx.attr.pkgbuild_name,
            "%LOG_LEVEL%": ctx.attr.log_level,
        },
    )

    folders = ctx.actions.declare_file(ctx.label.name + "-folders")
    ctx.actions.write(
        output = folders,
        content = """
# Provide folders as environment variables
export INSTALL_FOLDER="$3"
log_debug "INSTALL_FOLDER: $INSTALL_FOLDER"

BINARY_FOLDER="${{INSTALL_FOLDER}}{binary_folder}"
export BINARY_FOLDER="$(realpath $BINARY_FOLDER || $BINARY_FOLDER)"
log_debug "BINARY_FOLDER: '${{INSTALL_FOLDER}}{binary_folder}' resolved to '$BINARY_FOLDER'"

LOGS_FOLDER="${{INSTALL_FOLDER}}{logs_folder}"
export LOGS_FOLDER="$(realpath $LOGS_FOLDER || $LOGS_FOLDER)"
log_debug "LOGS_FOLDER: '${{INSTALL_FOLDER}}{logs_folder}' resolved to '$LOGS_FOLDER'"

RUN_FOLDER="${{INSTALL_FOLDER}}{run_folder}"
export RUN_FOLDER="$(realpath $RUN_FOLDER || $RUN_FOLDER)"
log_debug "RUN_FOLDER: '${{INSTALL_FOLDER}}{run_folder}' resolved to '$RUN_FOLDER'"
        """.format(binary_folder = ctx.attr.bin_folder, logs_folder = ctx.attr.logs_folder, run_folder = ctx.attr.run_folder),
    )

    ending = ctx.actions.declare_file(ctx.label.name + "-ending")
    ctx.actions.expand_template(
        template = ctx.file.ending,
        output = ending,
        substitutions = {
            "%NAME%": ctx.attr.pkgbuild_name,
        },
    )

    chunks_files = [prelude.path, folders.path] + [f.path for f in ctx.files.chunks] + [ending.path]

    ctx.actions.run_shell(
        inputs = ctx.files.chunks + [prelude, folders, ending],
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
        "bin_folder": attr.string(),
        "logs_folder": attr.string(),
        "run_folder": attr.string(),
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
