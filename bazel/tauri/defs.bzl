"""Provides a rule to run a Tauri application (frontend and backend)"""

def _tauri_run_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    all_files = [ctx.executable.backend, ctx.executable.frontend]

    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%{frontend_executable}": ctx.executable.frontend.short_path,
            "%{backend_executable}": ctx.executable.backend.short_path,
        },
        is_executable = True,
    )

    my_runfiles = ctx.runfiles(files = all_files)

    # Transitive dependencies:
    # my_runfiles = my_runfiles.merge(ctx.attr.test[DefaultInfo].default_runfiles)
    # for data in ctx.attr.data:
    #     my_runfiles = my_runfiles.merge(data[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            files = depset(all_files),
            runfiles = my_runfiles,
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
    },
    implementation = _tauri_run_impl,
    executable = True,
)
