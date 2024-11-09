"""An opinionated Bazel macro to create the targets for a Django project"""

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("@aspect_bazel_lib//lib:transitions.bzl", "platform_transition_filegroup")
load("@aspect_rules_py//py:defs.bzl", "py_binary", "py_library")
load("@rules_oci//oci:defs.bzl", "oci_load")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("//bazel/containers:py_layer.bzl", "py_oci_image")
load("//bazel/python/django/containers:defs.bzl", "DJANGO_PORT", "USER", "USER_UID")
load("//bazel/python/gunicorn:defs.bzl", "gunicorn_binary")

def django_project(name, settings_module, **kwargs):
    """
    An opinionated macro to create a Django project.

    Args:
        name(str): A name for the project
        settings_module(str): The module with the Django settings (i.e.: "project_name.settings")
        **kwargs(dict): Other arguments for the rules
    """

    env = kwargs.pop("env", {})
    env["DJANGO_SETTINGS_MODULE"] = settings_module

    deps = kwargs.pop("deps")

    py_library(
        name = "{}-wsgi".format(name),
        srcs = ["//bazel/python/django/project:wsgi.py"],
        deps = deps,
        **kwargs
    )

    py_library(
        name = "{}-asgi".format(name),
        srcs = ["//bazel/python/django/project:asgi.py"],
        deps = deps,
        **kwargs
    )

    py_binary(
        name = "{}-admin".format(name),
        srcs = ["//bazel/python/django/project:manage.py"],
        main = "manage.py",
        deps = [
            ":{}-wsgi".format(name),
        ],
        env = env,
        **kwargs
    )

    gunicorn_binary(
        name = "{}-gunicorn".format(name),
        args = [
            "bazel.python.django.project.wsgi:application",
            "--bind 0.0.0.0:{}".format(DJANGO_PORT),
            "--access-logfile '-'",
        ],
        env = env,
        deps = [
            ":{}-wsgi".format(name),
        ],
        **kwargs
    )

    #####
    # Create OCI image with this application ready to run
    #####

    expand_template(
        name = "{}-entrypoint-file".format(name),
        out = "entrypoint.sh",
        substitutions = {
            "%DJANGO_PORT%": DJANGO_PORT,
        },
        template = "//bazel/python/django/project:entrypoint.sh.tpl",
        is_executable = True,
    )

    pkg_tar(
        name = "{}-entrypoint".format(name),
        srcs = [":{}-entrypoint-file".format(name)],
    )

    py_oci_image(
        name = "{}-_container".format(name),
        base = "//bazel/python/django/containers/deploy",
        binaries = [
            ":{}-gunicorn".format(name),
            ":{}-admin".format(name),
        ],
        entrypoint = ["/entrypoint.sh"],
        exposed_ports = [DJANGO_PORT],
        tags = [
            "manual",
        ],
        tars = [
            ":{}-entrypoint".format(name),
        ],
        env = env | {
            # We need to override the database here so both 'admin' and 'gunicorn' use the same database
            "SQL_DATABASE": "/home/{}/web/db.sqlite3".format(USER),
        },
        user = USER_UID,
    )

    platform_transition_filegroup(
        name = "{}-container".format(name),
        srcs = [":{}-_container".format(name)],
        tags = [
            "manual",
        ],
        target_platform = select({
            "@platforms//cpu:arm64": "//bazel/platforms:linux_x86_64",
            "@platforms//cpu:x86_64": "//bazel/platforms:linux_x86_64",
        }),
    )

    # TODO: Some around this rule to test the container
    # container_structure_test(
    #     name = "structure_test",
    #     configs = ["test.yaml"],
    #     image = ":container",
    #     tags = [
    #         "ci",
    #         "manual",
    #     ],
    # )

    oci_load(
        name = "{}-load".format(name),
        image = ":{}-container".format(name),
        repo_tags = ["{}/{}:bazel-latest".format(native.package_name(), name)],  # FIXME: Create a file (stamped) to be used by all the repository
        tags = [
            "manual",
        ],
    )

    # We need this rule so 'oci_load' actually creates the tarball,
    # see https://github.com/bazel-contrib/rules_oci/blob/main/docs/load.md#build-outputs
    # for more info
    native.filegroup(
        name = "{}-tarball".format(name),
        srcs = [":{}-load".format(name)],
        output_group = "tarball",
        visibility = [":__subpackages__"],
    )
