"""Helper rules and macro to generate docker-compose files associated to Django deployments"""

load("@aspect_bazel_lib//lib:expand_template.bzl", "expand_template")
load("//bazel/containers/postgres:with_postgres_run.bzl", "POSTGRES_IMAGE_TAG")
load("//bazel/python/django/containers:defs.bzl", "DJANGO_PORT", "GROUP", "USER")

def docker_compose(name, app_repository, app_image_tag_stamped, **kwargs):
    expand_template(
        name = name,
        out = "docker-compose.yaml",
        substitutions = {
            "%APP_REPOSITORY%": app_repository,  # TODO: Better name
            "%USER%": USER,
            "%GROUP%": GROUP,
            "%APP_IMAGE_TAG%": native.module_name(),
            "%NGINX_DJANGO_TAG%": native.module_name(),
            "%ENV_FILE%": ".env.dev",
            "%ENV_DB_FILE%": ".env.dev.db",
            "%DJANGO_PORT%": DJANGO_PORT,
            "%POSTGRES_IMAGE_TAG%": POSTGRES_IMAGE_TAG,
        },
        stamp_substitutions = {
            "%APP_IMAGE_TAG%": app_image_tag_stamped,
            "%NGINX_DJANGO_TAG%": "{{STABLE_BUILD_SCM_REVISION}}",
            "%ENV_FILE%": ".env.prod",
            "%ENV_DB_FILE%": ".env.prod.db",
        },
        template = "//bazel/python/django/docker_compose:docker-compose.tpl.yaml",
        **kwargs
    )
