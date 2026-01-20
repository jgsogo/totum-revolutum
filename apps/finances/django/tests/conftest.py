import functools
import os
import subprocess
import tempfile

import docker
import pytest
import requests
from testcontainers.compose import DockerCompose


def _get_docker_host():
    """Get the Docker host URL from the current context or podman machine."""
    # Try to get from podman machine connection first (most reliable on macOS with Podman)
    try:
        result = subprocess.run(
            ["podman", "machine", "inspect", "--format", "{{.ConnectionInfo.PodmanSocket.Path}}"],
            capture_output=True,
            text=True,
            check=False,  # Don't raise on error
        )
        if result.returncode == 0:
            socket_path = result.stdout.strip()
            if socket_path and os.path.exists(socket_path):
                return f"unix://{socket_path}"
    except Exception as e:
        print(f"DEBUG: podman machine inspect failed: {e}")
    
    # Try common podman socket locations
    common_sockets = [
        "/var/folders/bk/ldcx_pkn4213f0xpzyl_v_bc0000gn/T/podman/podman-machine-default-api.sock",
        f"{os.path.expanduser('~')}/.local/share/containers/podman/machine/podman.sock",
        "/run/podman/podman.sock",
        "/var/run/docker.sock",
    ]
    
    for socket in common_sockets:
        if os.path.exists(socket):
            print(f"DEBUG: Found socket at: {socket}")
            return f"unix://{socket}"
    
    print("DEBUG: No docker/podman socket found")
    return None


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
    # Try to connect to Docker/Podman with different socket paths
    errors = []
    
    # Try default from_env() first
    try:
        client = docker.from_env()
        client.ping()
        yield client
        return
    except Exception as e:
        errors.append(f"from_env(): {e}")
    
    # Try explicit Docker socket
    try:
        client = docker.DockerClient(base_url='unix://var/run/docker.sock')
        client.ping()
        yield client
        return
    except Exception as e:
        errors.append(f"Docker socket: {e}")
    
    # Try Podman socket
    try:
        client = docker.DockerClient(base_url='unix:///run/podman/podman.sock')
        client.ping()
        yield client
        return
    except Exception as e:
        errors.append(f"Podman socket: {e}")
    
    # All attempts failed, raise error with details
    raise RuntimeError(
        "Cannot connect to Docker/Podman daemon. Tried:\n" + "\n".join(f"  - {err}" for err in errors)
    )


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
    
    # Set DOCKER_HOST to use the local podman socket instead of SSH
    docker_host = _get_docker_host()
    original_docker_host = os.environ.get("DOCKER_HOST")
    original_docker_context = os.environ.get("DOCKER_CONTEXT")
    
    print(f"DEBUG: docker_host from _get_docker_host(): {docker_host}")
    print(f"DEBUG: original DOCKER_HOST: {original_docker_host}")
    print(f"DEBUG: original DOCKER_CONTEXT: {original_docker_context}")
    
    if docker_host:
        os.environ["DOCKER_HOST"] = docker_host
        print(f"DEBUG: Set DOCKER_HOST to: {os.environ.get('DOCKER_HOST')}")
    # Unset DOCKER_CONTEXT to prevent it from overriding DOCKER_HOST
    os.environ.pop("DOCKER_CONTEXT", None)
    print(f"DEBUG: After unsetting, DOCKER_CONTEXT: {os.environ.get('DOCKER_CONTEXT')}")
    
    # Clean up any existing volumes from previous runs to avoid configuration conflicts
    try:
        subprocess.run(
            ["docker", "compose", "-f", "docker-compose.yaml", "down", "-v"],
            cwd=DOCKER_COMPOSE_PATH,
            capture_output=True,
            check=False,  # Don't fail if nothing to clean up
        )
    except Exception as e:
        print(f"DEBUG: Failed to clean up volumes (this is OK if first run): {e}")
    
    try:
        with DockerCompose(
            context=DOCKER_COMPOSE_PATH, compose_file_name="docker-compose.yaml", env_file=env_file
        ) as compose:
            yield compose
            stdout, stderr = compose.get_logs()
        print(">" * 50)
        print(stdout)
        print(stderr)
        print(">" * 50)
    finally:
        # Restore original environment
        if original_docker_host:
            os.environ["DOCKER_HOST"] = original_docker_host
        else:
            os.environ.pop("DOCKER_HOST", None)
        
        if original_docker_context:
            os.environ["DOCKER_CONTEXT"] = original_docker_context


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
