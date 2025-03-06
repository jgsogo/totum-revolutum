"""Some constant definitions for the finances installable"""

SUBSTITUTIONS = {
    "%NAME%": "finances",
    "%GUNICORN_APP%": "app-gunicorn",
    "%DJANGO_ADMIN_APP%": "app-admin",
}

path_selector = {
    "@platforms//os:linux": "/usr/local/finances",
    "@platforms//os:macos": "/usr/local/finances",
    # "@platforms//os:windows": "/Program Files/Finances",
    "//conditions:default": "/usr/local/finances",
}

DATA_FOLDER = select(path_selector) + "/data"
BIN_FOLDER = select(path_selector) + "/bin"

LOGS_FOLDER = "/usr/local/var/log"
RUN_FOLDER = "/usr/local/var/run"
