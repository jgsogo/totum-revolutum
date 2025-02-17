"""Provides a macro to run pytest"""

load("@aspect_rules_py//py:defs.bzl", "py_pytest_main", "py_test")
load("@py_deps//:requirements.bzl", "requirement")

def pytest_test(name, srcs, deps = [], args = [], test_args = [], **kwargs):
    py_pytest_main(
        name = "__test__{name}".format(name = name),
        deps = ["@py_deps//pytest:pkg"],
    )

    py_test(
        name = name,
        srcs = [
            ":__test__{name}".format(name = name),
        ] + srcs,
        main = ":__test__{name}.py".format(name = name),
        args = [
            "--capture=no",
            "-svv",
        ] + args + ["$(location %s)" % x for x in srcs] + test_args,
        deps = deps + [
            requirement("pytest"),
            # requirement("pytest-mock"),
            # requirement("pytest-timeout"),
            ":__test__{name}".format(name = name),
        ],
        **kwargs
    )
