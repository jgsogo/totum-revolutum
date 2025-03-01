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

def _collect_files_in_default_info(ctx, input):
    transitive_runfiles = []
    sources = []
    targets = []

    input_files = _install_files_depset(input[DefaultInfo])
    if not input_files:
        return transitive_runfiles, sources, targets

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

def _collect_files(ctx, files_attr):
    transitive_runfiles = []
    sources = []
    targets = []

    for input in files_attr:
        tr, s, t = _collect_files_in_default_info(ctx, input)
        transitive_runfiles = transitive_runfiles + tr
        sources = sources + s
        targets = targets + t

    return transitive_runfiles, sources, targets

def _deploy_local_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)

    # Collect files to copy
    transitive_runfiles, sources, targets = _collect_files(ctx, ctx.attr.data)
    transitive_runfiles_tars, sources_tars, targets_tars = _collect_files(ctx, ctx.attr.tars)
    transitive_runfiles_envs, sources_envs, targets_envs = _collect_files(ctx, ctx.attr.envs)

    untars = ["" for x in range(len(sources))] + ["1" for x in range(len(sources_tars))] + ["" for x in range(len(sources_envs))]
    envs = ["" for x in range(len(sources))] + ["" for x in range(len(sources_tars))] + ["1" for x in range(len(sources_envs))]

    sources = sources + sources_tars + sources_envs
    targets = targets + targets_tars + targets_envs
    transitive_runfiles = transitive_runfiles + transitive_runfiles_tars + transitive_runfiles_envs

    # If needed, copy envsubst tool too
    if len(sources_envs):
        transitive_runfiles_envsubst, sources_envsubst, targets_envsubst = _collect_files_in_default_info(ctx, ctx.attr._envsubst)
        sources = sources + sources_envsubst
        targets = targets + targets_envsubst
        transitive_runfiles = transitive_runfiles + transitive_runfiles_envsubst
        untars = untars + [""]
        envs = envs + [""]

    # The install file
    install_file_output = ctx.actions.declare_file(ctx.label.name + "_install.sh")
    ctx.actions.write(
        output = install_file_output,
        content = "$pwd",
        is_executable = True,
    )
    sources = sources + [paths.join(ctx.workspace_name, install_file_output.short_path)]
    targets = targets + [paths.join(ctx.attr.target_subdir, "install.sh")]
    untars = untars + [""]
    envs = envs + [""]

    # Render the template to do the copy
    ctx.actions.expand_template(
        template = ctx.file._run_template,
        output = executable,
        substitutions = {
            "%BASH_RLOCATION_FUNCTION%": BASH_RLOCATION_FUNCTION,
            "@@SOURCE_FILES@@": shell.array_literal(sources),
            "@@TARGET_NAMES@@": shell.array_literal(targets),
            "@@UNTARS@@": shell.array_literal(untars),
            "@@ENVS@@": shell.array_literal(envs),
            "@@INSTALLER_LABEL@@": shell.quote("@{}//{}:{}".format(
                ctx.workspace_name,
                ctx.label.package,
                # Strip leading '_' and suffix form the name.
                ctx.label.name,
            )),
        },
        is_executable = True,
    )

    runfiles = ctx.runfiles(
        files = [executable, install_file_output],
        transitive_files = depset(transitive = transitive_runfiles),
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
        "envs": attr.label_list(
            # mandatory = True,
            allow_files = True,
            doc = "Files with env-vars to deploy",
        ),
        "target_subdir": attr.string(default = ""),
        "_run_template": attr.label(
            default = Label("//bazel/python/django/deployment:deploy_local.tpl.sh"),
            allow_single_file = True,
        ),
        "_runfiles": attr.label(default = "@bazel_tools//tools/bash/runfiles"),
        "_envsubst": attr.label(default = "//bazel/tools/envsubst"),
    },
    doc = "Deploys the applications to the given folder",
    executable = True,
)

def deploy_local(name, admin, gunicorn, envs = [], **kwargs):
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
        envs = envs,
        tars = [
            ":{}-admin-tar".format(name),
            ":{}-gunicorn-tar".format(name),
        ],
        **kwargs
    )
