load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_foreign_cc//foreign_cc:defs.bzl", "cmake")

cc_library(
    name = "libpq",
    # hdrs = glob(["/usr/local/opt/libpq/include/**/*.h"]),
    includes = ["/usr/local/opt/libpq/include"],
    linkopts = ["-L/usr/local/opt/libpq/lib", "-lpq"],
    visibility = ["//visibility:public"],
)

filegroup(
    name = "all_srcs",
    srcs = glob(
        include = ["**"],
        # exclude = ["*.bazel"],
    ),
)

filegroup(
    name = "headers",
    srcs = glob(["include/pqxx/**/*"]),
)

cmake(
    name = "libpqxx_",
    build_args = [
        # "-Wno-error=unused-command-line-argument"
    ],
    cache_entries = {
        "BUILD_DOC": "off",
        "SKIP_BUILD_TEST": "on",
        "CMAKE_C_COMPILER_WORKS": "1",
        "CMAKE_CXX_COMPILER_WORKS": "1",
        # TODO: Take this from .bazelrc. Apply only to macos
        "CMAKE_OSX_DEPLOYMENT_TARGET": "15.0",
        "PostgreSQL_ROOT": "/usr/local/opt/libpq", # FIXME: This is a local (and hardcoded) path!
    },
    # These envs taken from # TODO: https://github.com/bazel-contrib/toolchains_llvm/issues/396
    env = {
        "AR": "$$EXT_BUILD_ROOT/external/toolchains_llvm~~llvm~llvm_toolchain/bin/llvm-ar",
        "STRIP": "$$EXT_BUILD_ROOT/external/toolchains_llvm~~llvm~llvm_toolchain/bin/llvm-strip",
        "CXXFLAGS": "-Wno-c++11-narrowing",
        "PKG_CONFIG_PATH": "/usr/local/opt/libpq/lib/pkgconfig:/usr/local/opt/openssl/lib/pkgconfig", # FIXME: This is a local (and hardcoded) path!
    },
    generate_crosstool_file = False,
    lib_source = ":all_srcs",
    # TODO: Use @postgres bazel module (https://registry.bazel.build/modules/postgres) to consume `pq` library from it
    linkopts = [
        "-L/usr/local/opt/libpq/lib", # FIXME: This is a local (and hardcoded) path!
        "-L/usr/local/opt/openssl/lib", # FIXME: This is a local (and hardcoded) path!
        "-lpq",
    ],
    out_static_libs = [
        "libpqxx.a",
    ],
    # visibility = ["//visibility:public"],
)

# The only purpose of this "alias" target is to provide a cc_library that the
# hedron_compile_commands will recognize to populate the compile_commands.json
cc_library(
    name = "libpqxx",
    hdrs = [":headers"],
    strip_include_prefix = "include",
    visibility = ["//visibility:public"],
    deps = [
        ":libpqxx_",
    ],
)
