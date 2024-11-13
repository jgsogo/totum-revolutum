from bazel.tools.semver.bump import VersionComponent, _bump_version


def test_bump_major():
    assert _bump_version("app-v0.0.0", VersionComponent.MAJOR, prerelease=False) == "app-v1.0.0"
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.MAJOR, prerelease=False) == "app-v1.0.0"
    )

    assert _bump_version("app-v0.0.0", VersionComponent.MAJOR, prerelease=True) == "app-v1.0.0-rc.1"
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.MAJOR, prerelease=True)
        == "app-v1.0.0-rc.1"
    )


def test_bump_minor():
    assert _bump_version("app-v0.0.0", VersionComponent.MINOR, prerelease=False) == "app-v0.1.0"
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.MINOR, prerelease=False) == "app-v0.1.0"
    )

    assert _bump_version("app-v0.0.0", VersionComponent.MINOR, prerelease=True) == "app-v0.1.0-rc.1"
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.MINOR, prerelease=True)
        == "app-v0.1.0-rc.1"
    )


def test_bump_patch():
    assert _bump_version("app-v0.0.0", VersionComponent.PATCH, prerelease=False) == "app-v0.0.1"
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.PATCH, prerelease=False) == "app-v0.0.1"
    )

    assert _bump_version("app-v0.0.0", VersionComponent.PATCH, prerelease=True) == "app-v0.0.1-rc.1"
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.PATCH, prerelease=True)
        == "app-v0.0.1-rc.1"
    )


def test_bump_prerelease():
    assert (
        _bump_version("app-v0.0.0", VersionComponent.PRERELEASE, prerelease=False)
        == "app-v0.0.1-rc.1"
    )  # We bump patch as well so we get a "greater" version
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.PRERELEASE, prerelease=False)
        == "app-v0.0.0-rc.2"
    )

    assert (
        _bump_version("app-v0.0.0", VersionComponent.PRERELEASE, prerelease=True)
        == "app-v0.0.1-rc.1"
    )  # We bump patch as well so we get a "greater" version
    assert (
        _bump_version("app-v0.0.0-rc.1", VersionComponent.PRERELEASE, prerelease=True)
        == "app-v0.0.0-rc.2"
    )
