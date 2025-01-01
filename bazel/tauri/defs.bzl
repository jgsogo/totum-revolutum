"""Provides a rule to run a Tauri application (frontend and backend)"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION", "to_rlocation_path")

def _tauri_run_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    all_files = [ctx.executable.backend, ctx.executable.frontend]

    frontend = ctx.attr.frontend.files_to_run.executable
    backend = ctx.attr.backend.files_to_run.executable

    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "%{frontend_executable}": to_rlocation_path(ctx, frontend),
            "%{backend_executable}": to_rlocation_path(ctx, backend),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = all_files, transitive_files = depset([]), collect_data = True)
    runfiles = runfiles.merge(ctx.attr.frontend.default_runfiles)
    runfiles = runfiles.merge(ctx.attr.backend.default_runfiles)
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    # Transitive dependencies:
    # my_runfiles = my_runfiles.merge(ctx.attr.test[DefaultInfo].default_runfiles)
    # for data in ctx.attr.data:
    #     my_runfiles = my_runfiles.merge(data[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            files = depset(all_files),
            runfiles = runfiles,
            executable = executable,
        ),
    ]

tauri_run = rule(
    attrs = {
        "backend": attr.label(
            doc = "The Tauri backend to run",
            executable = True,
            cfg = "target",
        ),
        "frontend": attr.label(
            doc = "The frontend executable",
            executable = True,
            cfg = "target",
        ),
        "_run_template": attr.label(
            default = Label("//bazel/tauri:run.sh.tpl"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    implementation = _tauri_run_impl,
    executable = True,
)
