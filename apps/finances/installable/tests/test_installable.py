import os
import subprocess
import xml.etree.ElementTree as ET
from pathlib import Path
from tempfile import TemporaryDirectory

import pytest


@pytest.fixture(scope="session")
def version():
    VERSION_FILE = os.getenv("TEST_SRCDIR") + "/_main/apps/finances/version.txt"
    assert os.path.isfile(VERSION_FILE), "VERSION_FILE is missing"
    return Path(VERSION_FILE).read_text().strip()


@pytest.fixture(scope="session")
def pkg_expanded_dir():
    PKG_FILE = os.getenv("TEST_SRCDIR") + "/_main/apps/finances/installable/installable.pkg"
    assert os.path.isfile(PKG_FILE), "PKG is missing"

    with TemporaryDirectory() as tmpdirname:
        temp_dir = Path(tmpdirname) / "pkg"
        proc = subprocess.Popen(
            ["pkgutil", "--expand", PKG_FILE, temp_dir],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        stdout, stderr = proc.communicate()

        yield temp_dir


@pytest.fixture(scope="session")
def pkg_info(pkg_expanded_dir):
    pkg_info = pkg_expanded_dir / "PackageInfo"
    assert os.path.isfile(pkg_info)

    tree = ET.parse(pkg_info)
    root = tree.getroot()
    yield root


@pytest.fixture(scope="session")
def preinstall(pkg_expanded_dir):
    preinstall = pkg_expanded_dir / "scripts" / "preinstall"
    assert os.path.isfile(preinstall)
    yield preinstall.read_text()


@pytest.fixture(scope="session")
def postinstall(pkg_expanded_dir):
    postinstall = pkg_expanded_dir / "scripts" / "postinstall"
    assert os.path.isfile(postinstall)
    yield postinstall.read_text()


def test_pkg_info(pkg_info, version):
    assert pkg_info.attrib["version"] == version
    assert pkg_info.attrib["identifier"] == "com.totum-revolutum.finances"


def test_preinstall(preinstall, version):
    assert 'INSTALLER_NAME="finances2"' in preinstall
    assert "Check free space" in preinstall
    assert "Check if nginx is running" in preinstall
    assert "Check if supervisord is running" in preinstall
    assert "Stop supervisor application" in preinstall


def test_postinstall(postinstall):
    assert "# Load the envvars" in postinstall  # We are loading an environment file
    assert "if ! check_db_connection; then" in postinstall  # No DB connection has consequences
    assert "if ! check_database_exists; then" in postinstall  # No DB has consequences
    assert "create_database" in postinstall  # We create the database
    assert "Execute Django command 'collectstatic" in postinstall
    assert "Execute Django command 'migrate'" in postinstall
    assert "Change owner of '$ROOT_FOLDER'" in postinstall
    assert "Change owner of '$MACOS_APPLICATION_FOLDER'" in postinstall
