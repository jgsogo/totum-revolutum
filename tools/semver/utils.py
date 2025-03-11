import re
from typing import List, Optional, Tuple

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


def sanitize_version_list(
    versions: List[str], filter_prefix: Optional[str] = None, raise_if_empty: Optional[bool] = False
) -> List[Tuple[str, semver.Version]]:
    """Sanitize a list of versions checking that all are valid Semver"""
    versions = [parse_version(v) for v in versions]
    if filter_prefix:
        versions = [(p, v) for p, v in versions if p == filter_prefix]

    if raise_if_empty and not versions:
        raise ValueError("Empty version list")

    prefixes = [p for (p, _) in versions]
    if prefixes and prefixes.count(prefixes[0]) != len(prefixes):
        raise ValueError("Not all versions have the same prefix")

    return versions
