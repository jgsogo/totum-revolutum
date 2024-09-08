"""Provides rules and macros related to the Tauri backend"""

load("@crates_libraries//:defs.bzl", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("//bazel/rust/ws:extract_from_workspace.bzl", "extract_from_workspace")

def tauri_backend(name, visibility):
    """Creates all the rules for the Rust application (backend) in Tarui app

    Args:
        name:
        visibility:
    """

    extract_from_workspace(
        name = "{}-extract".format(name),
        output_cargo_toml = "Cargo2.toml",
        package = ":Cargo.toml",
        workspace = "//:Cargo.toml",
    )

    cargo_build_script(
        name = "{}-cargo_build".format(name),
        srcs = ["build.rs"],
        build_script_env = {
            "DEP_TAURI_DEV": "false",
        },
        data = [
            "Cargo.toml",
            # "//:Cargo.toml",
            # ":extract",

            # TODO: ":extract" es un Cargo.toml sin workspace. Diría que aquí necesitamos una rule 'tauri_build_build' que coja este Cargo.toml y
            # haga lo que tenga que hacer COPIÁNDOSE los ficheros a otro directorio y ejecutando allí sus cosas.

            # La alternativa es generar lo que sea que genere 'tauri_build::build()' via Bazel

            # O también tenemos la opción de exportar más cosas: el root del workspace
            "tauri.conf.json",
        ] + native.glob(["src/**/*.rs"]),
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
