import pytest

from tools.semver.min_version import _min_version


def test_min_version():
    versions = ["app-v1.0.0", "app-v2.0.0"]

    v = _min_version(versions)
    assert v == "app-v1.0.0"


def test_empty_list():
    versions = []

    with pytest.raises(ValueError) as exc_info:
        _min_version(versions)

    assert str(exc_info.value) == "Empty version list"
