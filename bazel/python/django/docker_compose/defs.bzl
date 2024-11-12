"""Helper rules and macro to generate docker-compose files associated to Django deployments"""

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("//bazel/python/django/containers:defs.bzl", "DJANGO_PORT", "USER")

def docker_compose(name, app_image, **kwargs):
    expand_template(
        name = name,
        out = "docker-compose.yaml",
        substitutions = {
            "%APP_NAME%": app_image,  # TODO: Better name
            "%USER%": USER,
            "%APP_IMAGE%": app_image,
            "%APP_IMAGE_TAG%": "totum-revolutum",
            "%NGINX_DJANGO_TAG%": "totum-revolutum",
            "%ENV_FILE%": ".env.dev",
            "%ENV_DB_FILE%": ".env.dev.db",
            "%DJANGO_PORT%": DJANGO_PORT,
        },
        stamp_substitutions = {
            "%APP_IMAGE_TAG%": "{{STABLE_BUILD_SCM_REVISION}}",
            "%NGINX_DJANGO_TAG%": "{{STABLE_BUILD_SCM_REVISION}}",
            "%ENV_FILE%": ".env.prod",
            "%ENV_DB_FILE%": ".env.prod.db",
        },
        template = "//bazel/python/django/docker_compose:docker-compose.yaml.tpl",
        **kwargs
    )
