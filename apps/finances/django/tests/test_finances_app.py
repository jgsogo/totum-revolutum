import os

import docker

# from testcontainers.core.container import DockerContainer
from testcontainers.compose import DockerCompose

# import requests


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
        DOCKER_COMPOSE_PATH,
        compose_file_name=[
            "docker-compose.yaml",
        ],
        pull=False,
    ) as compose:
        # host = compose.get_service_host("web", 4444)
        # port = compose.get_service_port("web", 4444)

        # driver = webdriver.Remote(
        #     command_executor=("http://{}:{}/wd/hub".format(host,port)),
        #     desired_capabilities=CHROME,
        # )
        # driver.get("http://automation-remarks.com")
        stdout, stderr = compose.get_logs()

        print(">" * 50)
        print(stdout)
        print(stderr)
        # if stderr:
        #     print("Errors\\n:{}".format(stderr))

    # with DockerContainer(IMAGE_NAME).with_bind_ports(host=9000, container=8080) as container:
    #     # get_exposed_port waits for the container to be ready
    #     # https://github.com/testcontainers/testcontainers-python/blob/2bcb931063e84da1364aa26937778f0e45708000/core/testcontainers/core/container.py#L107-L108  # noqa: E501
    #     port = container.get_exposed_port(8080)
    #     assert port

    #     # TODO(alexeagle): have the application inside the container listen on a
    #     # port so we can use it as a test fixture
    #     r = requests.get("http://localhost:8080")
    #     print(r)

    assert 1 == 2
