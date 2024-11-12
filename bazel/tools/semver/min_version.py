import argparse
from typing import List

from .utils import compose_version, parse_version


def min_version(args: argparse.Namespace) -> str:
    """Returns the min version in a given list"""
    v = _min_version(args.versions)
    print(v)


def _min_version(versions: List[str]) -> str:
    """Returns the min version in a given list"""
    versions = [parse_version(v) for v in versions]
    prefixes = [p for (p, _) in versions]
    if prefixes.count(prefixes[0]) != len(prefixes):
        raise ValueError("Not all versions have the same prefix")

    version_objs = [v for (_, v) in versions]
    v = min(version_objs)

    return compose_version(prefix=prefixes[0], version=v)
