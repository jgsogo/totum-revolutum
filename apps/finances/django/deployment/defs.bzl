"""Shared bzl constants and methods for building the finances product."""

shared_object_path_selector = {
    "@platforms//os:linux": "/usr/share/finances",
    "@platforms//os:macos": "/Library/Finances",
    "@platforms//os:windows": "/Program Files/Finances",
    "//conditions:default": "/usr/local/share/finances",
}
