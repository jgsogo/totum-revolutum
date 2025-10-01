"""Some constant definitions for the finances installable"""

NAME = "finances2"
DJANGO_ADMIN_APP = "app-admin"

SUBSTITUTIONS = {
    "%NAME%": NAME,
    "%GUNICORN_APP%": "app-gunicorn",
    "%DJANGO_ADMIN_APP%": DJANGO_ADMIN_APP,
}

path_selector = {
    "@platforms//os:linux": "/usr/local/{}".format(NAME),
    "@platforms//os:macos": "/usr/local/{}".format(NAME),
    "//conditions:default": "/usr/local/{}".format(NAME),
}

ROOT_FOLDER = select(path_selector)
DATA_FOLDER = select(path_selector) + "/data"
BIN_FOLDER = select(path_selector) + "/bin"

LOGS_FOLDER = "/usr/local/{}/log".format(NAME)
RUN_FOLDER = "/usr/local/{}/run".format(NAME)
WWW_DATA_FOLDER = "/usr/local/{}/www-data".format(NAME)

MACOS_APPLICATION_FOLDER = "/Applications/Finances.app"
MACOS_APPLICATION_CONTENTS = "{}/Contents".format(MACOS_APPLICATION_FOLDER)
MACOS_APPLICATION_CONTENTS_RESOURCES = "{}/Contents/Resources".format(MACOS_APPLICATION_FOLDER)
MACOS_APPLICATION_CONTENTS_MACOS = "{}/Contents/MacOS".format(MACOS_APPLICATION_FOLDER)

NAME_QT = "finances-qt"
MACOS_APPLICATION_QT_FOLDER = "/Applications/FinancesQt.app"
MACOS_APPLICATION_QT_CONTENTS = "{}/Contents".format(MACOS_APPLICATION_QT_FOLDER)
MACOS_APPLICATION_QT_CONTENTS_RESOURCES = "{}/Contents/Resources".format(MACOS_APPLICATION_QT_FOLDER)
MACOS_APPLICATION_QT_CONTENTS_MACOS = "{}/Contents/MacOS".format(MACOS_APPLICATION_QT_FOLDER)

# ALL_FOLDERS = {
#     "LOGS_FOLDER": LOGS_FOLDER,
#     "BIN_FOLDER": BIN_FOLDER,
#     "RUN_FOLDER": RUN_FOLDER,
#     "DATA_FOLDER": DATA_FOLDER,
#     "WWW_DATA_FOLDER": WWW_DATA_FOLDER,
# }

ALL_FOLDERS = select(
    {
        "@platforms//os:linux": {},
        "@platforms//os:macos": {
            "ROOT_FOLDER": "/usr/local/{}".format(NAME),
            "LOGS_FOLDER": LOGS_FOLDER,
            "BIN_FOLDER": "/usr/local/{}/bin".format(NAME),
            "RUN_FOLDER": RUN_FOLDER,
            "DATA_FOLDER": "/usr/local/{}/data".format(NAME),
            "WWW_DATA_FOLDER": WWW_DATA_FOLDER,
            "MACOS_APPLICATION_FOLDER": MACOS_APPLICATION_FOLDER,
            "MACOS_APPLICATION_QT_FOLDER": MACOS_APPLICATION_QT_FOLDER,
        },
        # "@platforms//os:windows": "/Program Files/Finances",
        "//conditions:default": {},
    },
)
