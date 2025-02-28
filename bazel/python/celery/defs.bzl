"""Rules related to celery"""

load("@py_deps//:requirements.bzl", "requirement")
load("@rules_python//python:defs.bzl", "py_binary")

def celery_binary(name, deps, args, **kwargs):
    py_binary(
        name = name,
        srcs = ["//bazel/python/celery:celery_wrapper.py"],
        main = "//bazel/python/celery:celery_wrapper.py",
        deps = deps + [
            requirement("celery"),
            requirement("redis"),  # We could use other message queues
        ],
        args = args,
        **kwargs
    )
