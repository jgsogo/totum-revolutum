import pytest
from semver import Version

from tools.semver.utils import (
    compose_version,
    parse_version,
    sanitize_version_list,
)


def test_parse_version():
    prefix, version = parse_version("app-v10.2.3-rc.1")
    assert prefix == "app"
    assert version.major == 10
    assert version.minor == 2
    assert version.patch == 3
    assert version.prerelease == "rc.1"


def test_compose_version():
    v = Version.parse("3.2.0")
    assert compose_version("app", v) == "app-v3.2.0"

    v = Version.parse("3.2.0-pre1")
    assert compose_version("app", v) == "app-v3.2.0-pre1"


def test_empty_list():
    versions = []

    assert sanitize_version_list(versions, raise_if_empty=False) == []
    with pytest.raises(ValueError) as exc_info:
        sanitize_version_list(versions, raise_if_empty=True)

    assert str(exc_info.value) == "Empty version list"


def test_empty_list_filtered():
    versions = ["app-v1.0.0", "app-v2.0.0"]

    assert sanitize_version_list(versions, filter_prefix="other", raise_if_empty=False) == []
    with pytest.raises(ValueError) as exc_info:
        sanitize_version_list(versions, filter_prefix="other", raise_if_empty=True)

    assert str(exc_info.value) == "Empty version list"


def test_diff_prefixes():
    versions = ["app-v1.0.0", "app-v2.0.0", "other-v0.0.0"]

    with pytest.raises(ValueError) as exc_info:
        sanitize_version_list(versions)

    assert str(exc_info.value) == "Not all versions have the same prefix"


def test_diff_prefixes_filtered():
    versions = ["app-v1.0.0", "app-v2.0.0", "other-v0.0.0"]

    v = sanitize_version_list(versions, filter_prefix="app")
    v = [compose_version(p, v) for p, v in v]
    assert v == ["app-v1.0.0", "app-v2.0.0"]
