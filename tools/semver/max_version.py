import argparse
from typing import List, Optional

from .utils import compose_version, sanitize_version_list


def max_version(args: argparse.Namespace) -> str:
    """Returns the max version in a given list"""
    v = _max_version(args.versions, filter_prefix=args.filter_prefix)
    print(v)


def _max_version(versions: List[str], filter_prefix: Optional[str] = None) -> str:
    """Returns the max version in a given list"""
    versions = sanitize_version_list(
        versions=versions, filter_prefix=filter_prefix, raise_if_empty=True
    )

    version_objs = [v for (_, v) in versions]
    v = max(version_objs)

    return compose_version(prefix=versions[0][0], version=v)
