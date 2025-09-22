load("@rules_cc//cc:cc_library.bzl", "cc_library")

licenses(["notice"])  # MIT

cc_library(
    name = "date",
    hdrs = [
        "include/date/date.h",
    ],
    include_prefix = "date",
    strip_include_prefix = "include/date",
    visibility = ["//visibility:public"],
)

cc_library(
    name = "tz",
    srcs = [
        "src/tz.cpp",
    ],
    hdrs = [
        "include/date/tz.h",
    ],
    include_prefix = "date",
    strip_include_prefix = "include/date",
    visibility = ["//visibility:public"],
    deps = [":date"],
)

cc_library(
    name = "iso_week",
    hdrs = [
        "include/date/iso_week.h",
    ],
    include_prefix = "date",
    strip_include_prefix = "include/date",
    visibility = ["//visibility:public"],
    deps = [":date"],
)

cc_library(
    name = "julian",
    hdrs = [
        "include/date/julian.h",
    ],
    include_prefix = "date",
    strip_include_prefix = "include/date",
    visibility = ["//visibility:public"],
    deps = [":date"],
)

cc_library(
    name = "islamic",
    hdrs = [
        "include/date/islamic.h",
    ],
    include_prefix = "date",
    strip_include_prefix = "include/date",
    visibility = ["//visibility:public"],
    deps = [":date"],
)
