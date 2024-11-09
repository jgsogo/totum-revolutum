load("@aspect_rules_py//py:defs.bzl", "py_binary", "py_library")
load("//bazel/python/gunicorn:defs.bzl", "gunicorn_binary")

def django_project(name, settings_module, **kwargs):

    env = kwargs.pop("env", {})
    env["DJANGO_SETTINGS_MODULE"] = settings_module

    deps = kwargs.pop("deps")

    py_library(
        name = "{}-wsgi".format(name),
        srcs = ["//bazel/python/django/project:wsgi.py"],
        deps = deps,
        **kwargs,
    )

    py_library(
        name = "{}-asgi".format(name),
        srcs = ["//bazel/python/django/project:asgi.py"],
        deps = deps,
        **kwargs,
    )


    py_binary(
        name = "{}-runserver".format(name),
        srcs = ["//bazel/python/django/project:manage.py"],
        main = "manage.py",
        args = ["runserver"],
        deps = [
            ":{}-wsgi".format(name),
        ],
        env = env,
        **kwargs,
    )

    gunicorn_binary(
        name = "{}-gunicorn".format(name),
        args = [
            "bazel.python.django.project.wsgi:application",
            "--bind 0.0.0.0:8000",
            "--access-logfile '-'",
        ],
        env = env,
        deps = [
            ":{}-wsgi".format(name),
        ],
        **kwargs
    )
