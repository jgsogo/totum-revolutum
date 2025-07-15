import functools
import os
import tempfile

import docker
import pytest
import requests
from testcontainers.compose import DockerCompose


@pytest.fixture(scope="session")
def env_file():
    """This is the environment file provided to the docker compose"""
    with tempfile.NamedTemporaryFile(delete_on_close=False) as fp:
        fp.write(b"SECRET_KEY=testing-app")
        fp.write(b"SQL_DATABASE=hello_django_finances_tests")
        fp.close()
        yield fp.name


@pytest.fixture(scope="session")
def docker_client():
    yield docker.from_env()


def _load_latest_tarball(docker_tarball, docker_client):
    """
    testcontainers requires the image already loaded in the daemon, but rules_oci places
    a tar file in bazel-out.
    So, we load the latest tarball to Docker
    So that we run the test against the latest image
    """
    with open(docker_tarball, "rb") as f:
        docker_client.images.load(f)


@pytest.fixture(scope="session")
def docker_compose(django_image_loaded, nginx_image_loaded, env_file):
    DOCKER_COMPOSE_PATH = os.getenv("TEST_SRCDIR") + "/_main/apps/finances/django"
    with DockerCompose(
        context=DOCKER_COMPOSE_PATH, compose_file_name="docker-compose.yaml", env_file=env_file
    ) as compose:
        yield compose
        stdout, stderr = compose.get_logs()
    print(">" * 50)
    print(stdout)
    print(stderr)
    print(">" * 50)


@pytest.fixture(scope="session")
def django_image_loaded(docker_client):
    TAR_PATH = os.getenv("TEST_SRCDIR") + "/_main/apps/finances/django/app-load/tarball.tar"
    _load_latest_tarball(TAR_PATH, docker_client)


@pytest.fixture(scope="session")
def nginx_image_loaded(docker_client):
    TAR_PATH = (
        os.getenv("TEST_SRCDIR")
        + "/_main/bazel/python/django/containers/nginx/nginx-load/tarball.tar"
    )
    _load_latest_tarball(TAR_PATH, docker_client)


@pytest.fixture(scope="session")
def app_url(docker_compose):
    host, port = docker_compose.get_service_host_and_port("nginx", 80)
    return f"http://{host}:{port}/"  # FIXME: It's http call (non SSL)


@pytest.fixture(scope="session")
def session(app_url):

    def new_request(prefix, f, method, url, *args, **kwargs):
        return f(method, prefix + url, timeout=90, *args, **kwargs)

    s = requests.Session()
    s.request = functools.partial(new_request, app_url, s.request)
    return s
