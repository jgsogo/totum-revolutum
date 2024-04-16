""" Bazel module providing some helper functions for the Rust rules
"""

load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_doc_test", "rust_library", "rust_test")

def rust_library_tests_and_docs(name, all_features, **kwargs):
    """Creates a predefined set of targets for the given arguments.

    The generated targets are:
     * a main target using `rust_library(name="<name>", **kwargs)`
     * another `rust_library` using `all_features`
     * a `rust_test` unittesting target `tests`
     * a `rust_doc` documentation target `doc`
     * a `rust_doc_test` testing documentation target `doc/test`

    Args:
        name (str): the name of the generated targets
        all_features (List[str]): all the features available for the Rust crate
        **kwargs: other arguments to use for `rust_library`
    """
    rust_library(
        name = name,
        **kwargs
    )

    kwargs.pop("crate_features", None)
    kwargs.pop("visibility", None)

    rust_library(
        name = "{}/all".format(name),
        crate_features = all_features,
        crate_name = name,
        visibility = ["//visibility:private"],
        **kwargs
    )

    rust_test(
        name = "tests",
        crate = ":{}/all".format(name),
        crate_features = all_features,
        visibility = ["//visibility:private"],
    )

    rust_doc(
        name = "doc",
        crate = ":{}/all".format(name),
        visibility = ["//libraries:__pkg__"],
    )

    rust_doc_test(
        name = "doc/tests",
        crate = ":{}/all".format(name),
        visibility = ["//visibility:private"],
    )
