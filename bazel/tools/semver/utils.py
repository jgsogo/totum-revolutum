import re
from typing import Tuple

import semver

RE_VERSION = re.compile(r"^(?P<prefix>\w+)-v(?P<version_str>.*)$")

# def filter_prefix(versions: List[str], prefix: str) -> List[str]:
#     """Filter the given list of versions. It returns only the versions that contain the `prefix`
#     """
#     pass


# def remove_prefix(version: str, prefix: str) -> str:
#     """Returns the version without the given prefix. It raises if the prefix is not there
#     """
#     pass


def parse_version(version: str) -> Tuple[str, semver.Version]:
    """Parses a version into a 'semver.Version' object"""
    m = RE_VERSION.match(version)
    if not m:
        raise ValueError(f"Version '{version}' doesn't match patther {RE_VERSION}")

    prefix = m.group("prefix")
    version = semver.Version.parse(m.group("version_str"))
    return (prefix, version)


def compose_version(prefix: str, version: semver.Version) -> str:
    return f"{prefix}-v{version}"
