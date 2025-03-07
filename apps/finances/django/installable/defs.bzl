"""Some constant definitions for the finances installable"""

DJANGO_ADMIN_APP = "app-admin"

SUBSTITUTIONS = {
    "%NAME%": "finances",
    "%GUNICORN_APP%": "app-gunicorn",
    "%DJANGO_ADMIN_APP%": DJANGO_ADMIN_APP,
}

path_selector = {
    "@platforms//os:linux": "/usr/local/finances",
    "@platforms//os:macos": "/usr/local/finances",
    # "@platforms//os:windows": "/Program Files/Finances",
    "//conditions:default": "/usr/local/finances",
}

DATA_FOLDER = select(path_selector) + "/data"
BIN_FOLDER = select(path_selector) + "/bin"

LOGS_FOLDER = "/usr/local/finances/log"
RUN_FOLDER = "/usr/local/finances/run"
WWW_DATA_FOLDER = "/usr/local/finances/www-data"

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
            "LOGS_FOLDER": LOGS_FOLDER,
            "BIN_FOLDER": "/usr/local/finances/bin",
            "RUN_FOLDER": RUN_FOLDER,
            "DATA_FOLDER": "/usr/local/finances/data",
            "WWW_DATA_FOLDER": WWW_DATA_FOLDER,
        },
        # "@platforms//os:windows": "/Program Files/Finances",
        "//conditions:default": {},
    },
)
