load("@rules_foreign_cc//foreign_cc:defs.bzl", "cmake")

filegroup(
    name = "all_srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

cmake(
    name = "libpqxx",
    build_args = [
        # "-Wno-error=unused-command-line-argument"
    ],
    cache_entries = {
        "BUILD_DOC": "off",
        "SKIP_BUILD_TEST": "on",
        "CMAKE_C_COMPILER_WORKS": "1",
        "CMAKE_CXX_COMPILER_WORKS": "1",
    },
    # These envs taken from # TODO: https://github.com/bazel-contrib/toolchains_llvm/issues/396
    env = {
        "AR": "$$EXT_BUILD_ROOT/external/toolchains_llvm~~llvm~llvm_toolchain/bin/llvm-ar",
        "STRIP": "$$EXT_BUILD_ROOT/external/toolchains_llvm~~llvm~llvm_toolchain/bin/llvm-strip",
        "CXXFLAGS": "-Wno-c++11-narrowing",
    },
    generate_crosstool_file = False,
    lib_source = ":all_srcs",
    linkopts = [
        #   "-L/opt/homebrew/opt/libpq/lib",
        "-lpq",
    ],
    out_static_libs = [
        "libpqxx.a",
    ],
    visibility = ["//visibility:public"],
    deps = [],
)

# configure_make(
#     name = "libpqxx",
#     # These envs taken from # TODO: https://github.com/bazel-contrib/toolchains_llvm/issues/396
#     env = {
#         "AR": "$$EXT_BUILD_ROOT/external/toolchains_llvm~~llvm~llvm_toolchain/bin/llvm-ar",
#         "STRIP": "$$EXT_BUILD_ROOT/external/toolchains_llvm~~llvm~llvm_toolchain/bin/llvm-strip",
#     },
#     configure_options = [
#         "--disable-documentation",
#         "--disable-shared",
#     ],
#     lib_source = ":all_srcs",
#     linkopts = ["-lpq"],
#     out_static_libs = ["libpqxx.a"],
#     visibility = ["//visibility:public"],
# )
