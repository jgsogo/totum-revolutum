import pytest

from bazel.tools.semver.max_version import _max_version


def test_max_version():
    versions = ["app-v1.0.0", "app-v2.0.0"]

    v = _max_version(versions)
    assert v == "app-v2.0.0"


def test_max_version_with_prerelease():
    versions = ["app-v1.0.0", "app-v2.0.0", "app-v2.0.1-rc.1"]

    v = _max_version(versions)
    assert v == "app-v2.0.1-rc.1"


def test_empty_list():
    versions = []

    with pytest.raises(ValueError) as exc_info:
        _max_version(versions)

    assert str(exc_info.value) == "Empty version list"
