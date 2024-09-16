"""Provides rules and macros related to the Tauri backend"""

load("@crates_libraries//:defs.bzl", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("//bazel:copy_filegroups.bzl", "copy_filegroups")
load("//bazel/rust/ws:extract_from_workspace.bzl", "extract_from_workspace")

def tauri_backend(name, visibility):
    """Creates all the rules for the Rust application (backend) in Tarui app

    It assumes the _standard_ layout for a Tauri application

    Args:
        name:
        visibility:
    """

    WORKING_FOLDER = "wf-backend"

    native.filegroup(
        name = "{}-src".format(name),
        srcs = native.glob(["src/**/*.rs"], exclude = ["src/main.rs"], allow_empty = False),
    )

    native.filegroup(
        name = "{}-icons".format(name),
        srcs = native.glob(["icons/**"], allow_empty = False),
    )

    native.filegroup(
        name = "{}-capabilities".format(name),
        srcs = native.glob(["capabilities/**"], allow_empty = False),
    )

    native.filegroup(
        name = "{}-build_rs".format(name),
        srcs = ["build.rs"],
    )

    native.filegroup(
        name = "{}-tauri_conf_json".format(name),
        srcs = ["tauri.conf.json"],
    )

    extract_from_workspace(
        name = "{}-cargo_toml".format(name),
        output_cargo_toml = "{}/Cargo.toml".format(WORKING_FOLDER),
        package = ":Cargo.toml",
        workspace = "//:Cargo.toml",
    )

    copy_filegroups(
        name = "_{}-src".format(name),
        folder = WORKING_FOLDER,
        strip_prefix = native.package_name(),
        targeted_filegroups = [
            ":{}-src".format(name),
            ":{}-icons".format(name),
            ":{}-capabilities".format(name),
            # ":{}-cargo_toml".format(name),
            # ":{}-build_rs".format(name),
            ":{}-tauri_conf_json".format(name),
        ],
    )

    copy_filegroups(
        name = "_{}-build_rs".format(name),
        folder = WORKING_FOLDER,
        strip_prefix = native.package_name(),
        targeted_filegroups = [":{}-build_rs".format(name)],
    )

    # Excute cargo_build_script on the copied files
    # FIXME: I guess this is not using the right Cargo.toml file.
    cargo_build_script(
        name = "{}-cargo_build".format(name),
        srcs = [":_{}-build_rs".format(name)],
        # rundir = "./{}".format(WORKING_FOLDER),
        # rundir = "",
        build_script_env = {
            "DEP_TAURI_DEV": "false",
        },
        data = [
            ":_{}-src".format(name),
            ":{}-cargo_toml".format(name),
            # ":_{}-build_rs".format(name),
        ],
        compile_data = [
            ":_{}-build_rs".format(name),
        ],
        deps = [
            "@crates_libraries//:tauri-build",
        ],
    )

    rust_library(
        name = "{}-applib".format(name),
        srcs = ["src/lib.rs"],
        crate_name = "finances_app_lib",
        data = [
            "tauri.conf.json",
        ] + native.glob(["icons/**"]) + native.glob(["capabilities/**"]),
        proc_macro_deps = all_crate_deps(
            proc_macro = True,
        ),
        deps = all_crate_deps(
            normal = True,
        ) + [
            ":{}-cargo_build".format(name),
        ],
    )

    rust_binary(
        name = name,
        srcs = ["src/main.rs"],
        proc_macro_deps = all_crate_deps(
            proc_macro = True,
        ),
        visibility = visibility,
        deps = all_crate_deps(
            normal = True,
        ) + [
            ":{}-applib".format(name),
        ],
    )
