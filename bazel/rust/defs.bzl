""" Bazel module providing some helper functions for the Rust rules
"""

load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

def rust_library_tests_and_docs(name, all_features, test_data = None, **kwargs):
    """Creates a predefined set of targets for the given arguments.

    This macro generates the following targets:
     * Rust libraries:
        - `name`: the Rust crate without any feature enabled
        - `name/<key>`: one crate per entry in the `all_features` dictionary
        - `name/all_features`: a crate with all features enabled
     * Documentation target `doc`
     * Testing targets `tests` and `doc/tests` for documentation

    # FIXME: I'm passing a dictionary to `all_features` even if dependencies between features
    are already declared in the `Cargo.toml` file. It looks like those dependencies are not
    taken into account by the Bazel `rust_library`.

    Args:
        name (str): the name of the generated targets
        all_features (Dict[str, List]): a dictionary mapping an identifier to a list of features. One
            target named `<name>/<key>` enabling the list of features in the `<value>` will be created,
            for each of the entries in the dictionary.
        test_data (List): data files (and targets) to add to the `data` argument in `rust_test`
        **kwargs: other arguments to use for `rust_library`
    """

    # A target without any feature
    rust_library(
        name = name,
        **kwargs
    )

    # Targets created following user inputs
    collect_all_features = list()
    for key, value in all_features.items():
        rust_library(
            name = "{}/{}".format(name, key),
            crate_features = value,
            **kwargs
        )
        collect_all_features = collect_all_features + value

    # Target with all features enabled
    rust_library(
        name = "{}/all_features".format(name),
        crate_features = collect_all_features,
        crate_name = name,
        **kwargs
    )

    # Unittests
    rust_test(
        name = "tests",
        crate = ":{}/all_features".format(name),
        crate_features = collect_all_features,
        data = test_data,
    )

    # Integration tests
    deps = kwargs.pop("deps", None)
    rust_test_suite(
        name = "integration_tests",
        srcs = native.glob(["tests/**"]),
        data = test_data,
        deps = deps + [":{}/all_features".format(name)],
    )

    # Documentation
    rust_doc(
        name = "doc",
        crate = ":{}/all_features".format(name),
        visibility = ["//libraries:__pkg__"],
    )

    # Documentation - tests
    rust_doc_test(
        name = "doc/tests",
        crate = ":{}/all_features".format(name),
    )
