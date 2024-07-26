""" Bazel module providing some helper functions for the Rust rules
"""

load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

def rust_library_tests_and_docs(name, all_features = {}, test_data = None, test_deps = None, test_suite_deps = None, test_docs_deps = None, **kwargs):
    """Creates a predefined set of targets for the given arguments.

    This macro generates the following targets:
     * Rust libraries:
        - `name/vanilla`: the Rust crate without any feature enabled
        - `name/<key>`: one crate per entry in the `all_features` dictionary
        - `name`: the Rust library with all features enabled
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
        test_deps (List): dependencies only for the `rust_test` rule
        test_suite_deps (List): dependencies only for the `rust_test_suite` rule
        test_docs_deps (List): dependencies only for the `rust_doc_test` rule
        **kwargs: other arguments to use for `rust_library`
    """

    crate_name = kwargs.pop("crate_name", name)

    # A target without any feature
    rust_library(
        name = "{}/vanilla".format(name),
        crate_name = crate_name,
        visibility = ["//visibility:public"],
        **kwargs
    )

    # Targets created following user inputs
    collect_all_features = list()
    for key, value in all_features.items():
        rust_library(
            name = "{}/{}".format(name, key),
            crate_features = value,
            crate_name = crate_name,
            visibility = ["//visibility:public"],
            **kwargs
        )
        collect_all_features = collect_all_features + value

    # Target with all features enabled
    rust_library(
        name = name,
        crate_features = collect_all_features,
        crate_name = crate_name,
        visibility = ["//visibility:public"],
        **kwargs
    )

    # Unittests
    rust_test(
        name = "tests",
        crate = ":{}".format(name),
        crate_features = collect_all_features,
        data = test_data,
        deps = test_deps,
    )

    # Integration tests
    deps = kwargs.pop("deps", None)
    test_suite_deps = test_suite_deps or []
    rust_test_suite(
        name = "integration_tests",
        crate_features = collect_all_features,
        srcs = native.glob(["tests/**/test_*.rs"]),
        data = test_data,
        deps = deps + test_suite_deps + [":{}".format(name)],
    )

    # Documentation
    rust_doc(
        name = "doc",
        crate = ":{}".format(name),
        visibility = ["//visibility:public"],
    )

    # Documentation - tests
    test_docs_deps = test_docs_deps or []
    rust_doc_test(
        name = "doc/tests",
        crate = ":{}".format(name),
        deps = deps + test_docs_deps,
    )
