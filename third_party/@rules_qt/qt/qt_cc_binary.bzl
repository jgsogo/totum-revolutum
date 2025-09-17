"""
"""

load("@rules_cc//cc:cc_binary.bzl", "cc_binary")

def qt_cc_binary(name, srcs, deps = None, copts = [], data = [], env = {}, **kwargs):
    cc_binary(
        name = name,
        srcs = srcs,
        deps = deps,
        copts = copts + select({
            "@platforms//os:windows": [],
            "//conditions:default": ["-fPIC"],
        }),
        data = data + select({
            "@bazel_tools//src/conditions:darwin_x86_64": ["@qt_6.10.0_mac_x86_64//:qt_env"],
        }),
        env = env,
        # env = select({
        #     # "@platforms//os:linux": linux_env_data,
        #     "@rules_qt//:osx_x86_64": mac_x64_env_data,
        #     # "@rules_qt//:osx_arm64": mac_m1_env_data,
        #     # "@platforms//os:windows": windows_env_data,
        # }),
        **kwargs
    )
