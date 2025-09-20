load("@rules_cc//cc:cc_library.bzl", "cc_library")


def qt_cc_library(name, srcs, hdrs, normal_hdrs = [], copts = [], **kwargs):
    """Compiles a QT library and generates the MOC for it.

    Args:
      name: A name for the rule.
      srcs: The cpp files to compile.
      hdrs: The header files that the MOC compiles to src.
      normal_hdrs: Headers which are not sources for generated code.
      copts: cc_library copts
      **kwargs: Any additional arguments are passed to the cc_library rule.
    """
    _moc_srcs = []
    for hdr in hdrs:
        header_path = "%s/%s" % (native.package_name(), hdr) if len(native.package_name()) > 0 else hdr
        moc_name = "moc_%s" % hdr.rsplit(".", 1)[0]
        native.genrule(
            name = moc_name,
            srcs = [hdr],
            outs = [moc_name + ".cpp"],
            cmd = "$(location @rules_qt//:moc) $(locations %s) -o $@ -f'%s'" % (hdr, header_path),
            tools = ["@rules_qt//:moc"],
            tags = ["manual"],
        )
        _moc_srcs.append(":" + moc_name)

    cc_library(
        name = name,
        srcs = srcs + _moc_srcs,
        hdrs = hdrs + normal_hdrs,
        textual_hdrs = _moc_srcs,
        copts = copts + select({
            "@platforms//os:windows": [],
            "//conditions:default": ["-fPIC"],
        }),
        **kwargs
    )
