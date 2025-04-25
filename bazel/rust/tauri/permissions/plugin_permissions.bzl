# CargoBuildScriptRunfilesInfo = provider(
#     doc = "Info about a `cargo_build_script.script` target.",
#     fields = {
#         "data": "List[Target]: The raw `cargo_build_script_runfiles.data` attribute.",
#         "tools": "List[Target]: The raw `cargo_build_script_runfiles.tools` attribute.",
#     },
# )

def _plugin_permissions_impl(ctx):
    print(">>>>>>")
    print("build_script: {}".format(ctx.attr.build_script))
    print("inner_path: {}".format(ctx.attr.inner_path))
    print("<<<<<<")

    # return [
    #     DefaultInfo(
    #         files = depset([exe]),
    #         runfiles = runfiles.merge(ctx.attr.script[DefaultInfo].default_runfiles),
    #     ),
    # ]

plugin_permissions = rule(
    implementation = _plugin_permissions_impl,
    attrs = {
        "build_script": attr.label(
            mandatory = True,
        ),
        "inner_path": attr.string(
            doc = "Path inside the permissions directory that corresponds to this plugin",
        ),
    },
    doc = "Collect the permissions files for a Tauri plugin",
)
