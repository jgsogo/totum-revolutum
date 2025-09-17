"""
Some constants we are using across the project
"""

VERSIONS = [
    "6.10.0",
    "6.9.2",
    "6.8.3",  # LTS release
]
DEFAULT_VERSION = "6.8.3"  # Latest LTS

HOSTS = ["mac"]
TARGET_SDKS = ["desktop"]
ARCHS = ["clang_64"]

QT_RELEASES = [
    ("mac", "desktop", "clang_64"),
]

OUTPUT_DIR_FOR_HOST = {"mac": "macos"}

# List of targets that appear on every version/host/target_sdk/arch
QT_COMMON_LIBRARIES = [
    "qt",
    "qt_hdrs",
    "qt_env",
    "plugin_files",
    "qml_files",
    "uic",
    "moc",
    "rcc",
    "repo_rootpath",

    # Individual libraries
    "qt_core",
    "qt_widgets",
]

# aqt install-qt --help
#
#   {linux,linux_arm64,mac,windows,windows_arm64,all_os}
#                         host os name
#   {desktop,winrt,android,ios,wasm,qt}
#                         Target SDK
#   (VERSION | SPECIFICATION)
#                         Qt version in the format of "5.X.Y" or SimpleSpec like "5.X" or "<6.X"
#   arch
#                         target linux/desktop: linux_gcc_64, gcc_64, wasm_32
#                         target mac/desktop:   clang_64, wasm_32
#                         target mac/ios:       ios
#                         windows/desktop:      win64_msvc2022_64
#                                               win64_msvc2019_64, win32_msvc2019
#                                               win64_msvc2017_64, win32_msvc2017
#                                               win64_msvc2015_64, win32_msvc2015
#                                               win64_mingw81, win32_mingw81
#                                               win64_mingw73, win32_mingw73
#                                               win32_mingw53
#                                               wasm_32
#                         windows/winrt:        win64_msvc2019_winrt_x64, win64_msvc2019_winrt_x86
#                                               win64_msvc2017_winrt_x64, win64_msvc2017_winrt_x86
#                                               win64_msvc2019_winrt_armv7
#                                               win64_msvc2017_winrt_armv7
#                         android:              Qt 5.14:          android (optional)
#                                               Qt 5.13 or below: android_x86_64, android_arm64_v8a
#                                                                 android_x86, android_armv7
#                         all_os/wasm:          wasm_singlethread, wasm_multithread
