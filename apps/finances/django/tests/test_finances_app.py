import os

import docker
import requests

# from testcontainers.core.container import DockerContainer
from testcontainers.compose import DockerCompose

IMAGE_NAME = "django_finances:latest"


def _load_latest_tarball():
    """
    testcontainers requires the image already loaded in the daemon, but rules_oci places
    a tar file in bazel-out.
    So, we load the latest tarball to Docker
    So that we run the test against the latest image
    """
    TAR_PATH = os.getenv("TEST_SRCDIR") + "/_main/apps/finances/django/load/tarball.tar"
    client = docker.from_env()
    with open(TAR_PATH, "rb") as f:
        client.images.load(f)


def _load_postgres_tarball():
    TAR_PATH = os.getenv("TEST_SRCDIR") + "/_main/containers/postgres/tarball.tar"
    client = docker.from_env()
    with open(TAR_PATH, "rb") as f:
        client.images.load(f)


def test_app():
    _load_latest_tarball()
    _load_postgres_tarball()

    user = os.environ["USER"]
    assert user

    DOCKER_COMPOSE_PATH = os.getenv("TEST_SRCDIR") + "/_main/apps/finances/django"
    with DockerCompose(
        context=DOCKER_COMPOSE_PATH, compose_file_name="docker-compose.yaml"
    ) as compose:
        print(">" * 50)
        import time

        time.sleep(20)  # FIXME: Add some wait-until-ready patter (in docker itself)
        host, port = compose.get_service_host_and_port("web", 8080)
        print(host)
        print(port)

        import ssl

        print(ssl.OPENSSL_VERSION)

        url = f"http://{host}:{port}/data"  # FIXME: It's http call (non SSL)
        print(f"url: {url}")
        r = requests.get(url)
        assert r.status_code == 200

        # print(r)
        # import selenium
        # driver = webdriver.Remote(
        #     command_executor=("http://{}:{}/wd/hub".format(host,port)),
        #     desired_capabilities=selenium.CHROME,
        # )
        # r = driver.get("http://localhost:8000/data")
        print(r)
        stdout, stderr = compose.get_logs()

    print(">" * 50)
    print(stdout)
    print(stderr)

    # assert 1 == 2
