"""
"""

load("@rules_cc//cc:cc_binary.bzl", "cc_binary")

def qt_cc_binary(name, srcs, deps = None, copts = [], data = [], env = {}, toolchains = [], linkopts = [], **kwargs):
    cc_binary(
        name = name,
        srcs = srcs,
        deps = deps,
        copts = copts + select({
            "@platforms//os:windows": [],
            "//conditions:default": ["-fPIC"],
        }),
        data = data + ["@rules_qt//:qt_env"],
        env = env,
        linkopts = linkopts + [
            "-rpath $(REPO_ROOTPATH)",
        ],
        toolchains = toolchains + ["@rules_qt//:repo_rootpath"],
        # env = select({
        #     # "@platforms//os:linux": linux_env_data,
        #     "@rules_qt//:osx_x86_64": mac_x64_env_data,
        #     # "@rules_qt//:osx_arm64": mac_m1_env_data,
        #     # "@platforms//os:windows": windows_env_data,
        # }),
        **kwargs
    )
