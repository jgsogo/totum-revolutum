"""Rules related to gunicorn"""

load("@py_deps//:requirements.bzl", "requirement")
load("@rules_python//python:defs.bzl", "py_binary")

def gunicorn_binary(name, deps, args, **kwargs):
    """
    Creates a python binary to run gunicorn

    This binary contains all the sources from the project, this
    way the WSGI application is accessible to gunicorn
    """
    py_binary(
        name = name,
        srcs = ["//bazel/python/gunicorn:gunicorn_wrapper.py"],
        main = "//bazel/python/gunicorn:gunicorn_wrapper.py",
        deps = deps + [
            requirement("gunicorn"),
        ],
        args = args,
        **kwargs
    )
