"""An opinionated Bazel macro to create the targets for a Django project"""

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("@aspect_bazel_lib//lib:transitions.bzl", "platform_transition_filegroup")
load("@aspect_rules_py//py:defs.bzl", "py_library")
load("@py_deps//:requirements.bzl", "requirement")
load("@rules_oci//oci:defs.bzl", "oci_load")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("//bazel:sh_with_runfiles_binary.bzl", "sh_with_runfiles_binary")
load("//bazel/containers:py_layer.bzl", "py_oci_image")
load("//bazel/python/django/containers:defs.bzl", "DJANGO_PORT", "USER", "USER_UID")
load("//bazel/python/django/project:django_admin.bzl", "django_admin")
load("//bazel/python/django/project:django_gunicorn.bzl", "django_gunicorn")

def django_project(name, deps, srcs, **kwargs):
    """
    An opinionated macro to create a Django project.

    Args:
        name(str): A name for the project
        deps(List[str]): List of dependencies.
        srcs(List[str]): List of sources to include in the project.
        **kwargs(dict): Other arguments for the rules
    """
    settings_module = "{}.settings".format(native.package_name().replace("/", "."))

    env = kwargs.pop("env", {})
    env["DJANGO_SETTINGS_MODULE"] = settings_module
    env["DJANGO_ALLOWED_HOSTS"] = "localhost 127.0.0.1 0.0.0.0 [::1]"
    env["DEBUG"] = "1"
    env["SECRET_KEY"] = "4niv*0w++!1y%x59x(ma165cni2-0%m-jmx-7rpav1zmgp#no9-{}".format(settings_module)

    # The library, with all the application files
    py_library(
        name = "{}-project".format(name),
        srcs = srcs,
        imports = ["."],
        deps = deps + [
            requirement("django"),
            requirement("psycopg"),  # Required because deployments can use Postgres
        ],
    )

    # Django admin
    django_admin(
        name = "{}-admin".format(name),
        django_project = ":{}-project".format(name),
        settings = settings_module,
        tags = ["manual"],
        **kwargs
    )

    sh_with_runfiles_binary(
        name = "{}-migrate".format(name),
        args = [
            "migrate",
        ],
        tags = ["manual"],
        tool = ":{}-admin".format(name),
        **kwargs
    )

    # Gunicorn
    py_library(
        name = "{}-wsgi".format(name),
        srcs = ["//bazel/python/django/project:wsgi.py"],
        deps = [":{}-project".format(name)],
        **kwargs
    )

    py_library(
        name = "{}-asgi".format(name),
        srcs = ["//bazel/python/django/project:asgi.py"],
        deps = [":{}-project".format(name)],
        **kwargs
    )

    django_gunicorn(
        name = "{}-gunicorn".format(name),
        django_project = ":{}-wsgi".format(name),
        settings = settings_module,
        tags = ["manual"],
        **kwargs
    )

def django_project_container(name, version, repository, env = None, entrypoint = None, visibility = None):
    """Create OCI image with this application ready to run

    Args:
        name(str): Name of the created target
        version(Target): The target that contains the application version
        repository(str): Docker repository for the app image. Example: 'gcr.io/jgsogo/finances_app'
        env(Dict[str, str]): Environment variables
        entrypoint(Target): A pkg_tar target with (at least) the `entrypoint.sh` script to execute
        visibility: Visibility argument for Bazel targets
    """

    settings_module = "{}.settings".format(native.package_name().replace("/", "."))

    env = env or {}
    env["DJANGO_SETTINGS_MODULE"] = settings_module
    env["DJANGO_ALLOWED_HOSTS"] = "localhost 127.0.0.1 0.0.0.0 [::1]"
    env["DEBUG"] = "1"
    env["SECRET_KEY"] = "4niv*0w++!1y%x59x(ma165cni2-0%m-jmx-7rpav1zmgp#no9-{}".format(settings_module)

    # If there is no default entrypoint, we provide one by default
    if not entrypoint:
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

        entrypoint = ":{}-entrypoint".format(name)

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
            "no-remote-cache",
        ],
        tars = [
            entrypoint,
        ],
        env = env | {
            # We need to override the database here so both 'admin' and 'gunicorn' use the same database
            "SQL_DATABASE": "/home/{}/web/db.sqlite3".format(USER),
        },
        user = USER_UID,
        # Docs about the docker LABEL: https://docs.docker.com/reference/dockerfile/#label
        labels = {
            "org.opencontainers.image.source": "https://github.com/jgsogo/totum-revolutum",  # Associates this container with this repository # FIXME: Make it global for all the repo
        },
    )

    platform_transition_filegroup(
        name = "{}-container".format(name),
        srcs = [":{}-_container".format(name)],
        tags = [
            "manual",
        ],
        target_platform = select({
            "@platforms//cpu:arm64": "//bazel/platforms:linux_arm64",
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

    native.genrule(
        name = "{}-repo_tags".format(name),
        outs = ["repo_tags.txt"],
        cmd_bash = """
            echo "{}:$$(cat $(location {}))" > $@
        """.format(repository, version),
        srcs = [version],
        tags = ["manual"],
    )

    oci_load(
        name = "{}-load".format(name),
        image = ":{}-container".format(name),
        repo_tags = ":{}-repo_tags".format(name),
        tags = [
            "manual",
            "no-remote-cache",
        ],
    )

    # We need this rule so 'oci_load' actually creates the tarball,
    # see https://github.com/bazel-contrib/rules_oci/blob/main/docs/load.md#build-outputs
    # for more info
    native.filegroup(
        name = "{}-tarball".format(name),
        srcs = [":{}-load".format(name)],
        output_group = "tarball",
        visibility = visibility,
        tags = ["manual"],
    )
