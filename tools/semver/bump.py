import argparse
from enum import Enum

from .utils import compose_version, parse_version


class VersionComponent(Enum):
    MAJOR = 1
    MINOR = 2
    PATCH = 3
    PRERELEASE = 4


def bump_version(args: argparse.Namespace):
    component = VersionComponent[args.component.upper()]
    version = _bump_version(version=args.version, component=component, prerelease=args.prerelease)
    print(version)


def _bump_version(version: str, component: VersionComponent, prerelease: bool):
    prefix, version = parse_version(version=version)
    prerelease = prerelease or component == VersionComponent.PRERELEASE

    match component:
        case VersionComponent.MAJOR:
            version = version.bump_major()
        case VersionComponent.MINOR:
            version = version.bump_minor()
        case VersionComponent.PATCH:
            version = version.bump_patch()
        case VersionComponent.PRERELEASE:
            if not version.prerelease:
                version = version.bump_patch()
        case _:
            raise ValueError(f"Unknow version component '{component}'")

    if prerelease:
        version = version.bump_prerelease()

    return compose_version(prefix=prefix, version=version)
