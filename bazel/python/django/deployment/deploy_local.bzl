"""
Rule implementation to make a django deployment

Credit: https://github.com/google/bazel_rules_install/blob/main/installer/def.bzl
"""

load("@aspect_bazel_lib//lib:paths.bzl", "BASH_RLOCATION_FUNCTION")
load("@bazel_skylib//lib:paths.bzl", "paths")
load("@bazel_skylib//lib:shell.bzl", "shell")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

def _install_files_depset(default_info):
    direct = []
    transitive = []
    if default_info.files:
        transitive.append(default_info.files)
    if default_info.default_runfiles:
        transitive.append(default_info.default_runfiles.files)
    if default_info.files_to_run and default_info.files_to_run.executable:
        direct = [default_info.files_to_run.executable]
    if direct or transitive:
        return depset(direct = direct, transitive = transitive)
    return None

def _collect_files(ctx, files_attr):
    transitive_runfiles = []
    sources = []
    targets = []

    for input in files_attr:
        input_files = _install_files_depset(input[DefaultInfo])
        if not input_files:
            continue
        transitive_runfiles.append(input_files)

        workspace = (input.label.workspace_name or ctx.workspace_name)
        external_prefix = "external/"

        for file in input_files.to_list():
            file_path = file.path
            root_path = file.root.path + "/"

            if file_path.startswith(root_path):
                file_path = file_path[len(root_path):]
                file_path = workspace + "/" + file_path
                sources.append(file_path)

            elif file_path.startswith(external_prefix):
                file_path = file_path[len(external_prefix):]
                sources.append(file_path)

            else:
                file_path = workspace + "/" + file_path
                sources.append(file_path)

            target = paths.join(ctx.attr.target_subdir, paths.basename(file.short_path))
            targets.append(target)

    return transitive_runfiles, sources, targets

def _deploy_local_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Collect files to copy
    transitive_runfiles, sources, targets = _collect_files(ctx, ctx.attr.data)
    transitive_runfiles_tars, sources_tars, targets_tars = _collect_files(ctx, ctx.attr.tars)
    untars = ["" for x in range(len(sources))] + ["1" for x in range(len(sources_tars))]

    # Takes a target directory from the command line

    # Copies the admin, gunicorn, celery,... to the output directory

    # Creates the environment file

    # Creates the supervisord file

    # Creates the nginx file

    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "@@SOURCE_FILES@@": shell.array_literal(sources + sources_tars),
            "@@TARGET_NAMES@@": shell.array_literal(targets + targets_tars),
            "@@UNTARS@@": shell.array_literal(untars),
            "@@INSTALLER_LABEL@@": shell.quote("@{}//{}:{}".format(
                ctx.workspace_name,
                ctx.label.package,
                # Strip leading '_' and suffix form the name.
                ctx.label.name,
            )),

            # "%NAME%": ctx.label.package.replace("/", "_"),
            # "%GUNICORN_APP%": "the-gunicorn-app",
            # "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            # "%TOOL%": to_rlocation_path(ctx, tool),
            # "%TOOL_ARGS%": " ".join(args),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles(
        files = [executable],
        transitive_files = depset(transitive = transitive_runfiles + transitive_runfiles_tars),
    )
    runfiles = runfiles.merge(ctx.attr._runfiles.default_runfiles)

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

deploy_local_sh = rule(
    _deploy_local_impl,
    attrs = {
        # "admin": attr.label(
        #     doc = "Django admin application",
        #     mandatory = True,
        # ),
        # "gunicorn": attr.label(
        #     doc = "Django gunicorn application",
        #     mandatory = True,
        # ),
        # "celery": attr.label(
        #     doc = "Django gunicorn application",
        # ),
        "data": attr.label_list(
            # mandatory = True,
            allow_files = True,
            doc = "Plain files to deploy",
        ),
        "tars": attr.label_list(
            # mandatory = True,
            allow_files = True,
            doc = "Tar files to deploy, they will be unzipped in the target directory",
        ),
        "target_subdir": attr.string(default = ""),
        "_run_template": attr.label(
            default = Label("//bazel/python/django/deployment:deploy_local.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
    },
    doc = "Deploys the applications to the given folder",
    executable = True,
)

def deploy_local(name, admin, gunicorn, **kwargs):
    # FIXME: We need to create a tar file per application until this is fixed: https://github.com/bazelbuild/rules_pkg/issues/902
    pkg_tar(
        name = "{}-admin-tar".format(name),
        srcs = [
            admin,
        ],
        include_runfiles = True,
        mode = "0755",
        tags = ["manual"],
    )

    pkg_tar(
        name = "{}-gunicorn-tar".format(name),
        srcs = [
            gunicorn,
        ],
        include_runfiles = True,
        mode = "0755",
        tags = ["manual"],
    )

    deploy_local_sh(
        name = name,
        # admin = ":{}-admin-tar".format(name),
        # gunicorn = ":{}-gunicorn-tar".format(name),
        # data = [
        #     ":{}-admin-tar".format(name),
        #     ":{}-gunicorn-tar".format(name),
        # ],
        tars = [
            ":{}-admin-tar".format(name),
            ":{}-gunicorn-tar".format(name),
        ],
        **kwargs
    )
