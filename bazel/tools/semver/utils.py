import re
from typing import Tuple

import semver

RE_VERSION = re.compile(r"^(?P<prefix>\w+)-v(?P<version_str>.*)$")


def parse_version(version: str) -> Tuple[str, semver.Version]:
    """Parses a version into a 'semver.Version' object"""
    version = version.strip("\"'")
    m = RE_VERSION.match(version)
    if not m:
        raise ValueError(f"Version '{version}' doesn't match patther {RE_VERSION}")

    prefix = m.group("prefix")
    version = semver.Version.parse(m.group("version_str"))
    return (prefix, version)


def compose_version(prefix: str, version: semver.Version) -> str:
    return f"{prefix}-v{version}"
