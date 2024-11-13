import argparse
import logging
import sys

from bazel.tools.semver.bump import VersionComponent, bump_version
from bazel.tools.semver.max_version import max_version
from bazel.tools.semver.min_version import min_version

logger = logging.getLogger(__name__)

if __name__ == "__main__":

    parser = argparse.ArgumentParser(
        description="Tool to work with Semver versions (specific to this repo)",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="Verbosity level (defaults to ERROR). Add more -v to increase "
        "log level to WARNING, INFO or DEBUG.",
    )

    subparsers = parser.add_subparsers(title="subcommands", required=True)

    # Bump a version
    bump_parser = subparsers.add_parser(
        "bump",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Bumps a component of a version",
        help="Bumps a component of a version",
    )
    bump_parser.add_argument("version")
    bump_parser.add_argument(
        "--component",
        type=str,
        default="major",
        choices=[i.name.lower() for i in VersionComponent],
        help="Which version component to bump",
        required=True,
    )
    bump_parser.set_defaults(func=bump_version)

    # Compute min version
    min_version_parser = subparsers.add_parser(
        "min",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Returns the minimal version in the given list",
        help="Returns the minimal version in the given list",
    )
    min_version_parser.add_argument("versions", nargs="+", default=[])
    min_version_parser.add_argument(
        "--filter-prefix",
        type=str,
        help="If provided, it will consider only versions with the given prefix",
    )
    min_version_parser.set_defaults(func=min_version)

    # Compute max version
    max_version_parser = subparsers.add_parser(
        "max",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Returns the maximum version in the given list",
        help="Returns the maximum version in the given list",
    )
    max_version_parser.add_argument("versions", nargs="+", default=[])
    max_version_parser.add_argument(
        "--filter-prefix",
        type=str,
        help="If provided, it will consider only versions with the given prefix",
    )
    max_version_parser.set_defaults(func=max_version)

    args = parser.parse_args()

    # Configure log verbosity for all instantiated loggers
    min_log_level = logging.ERROR  # CRITICAL and ERROR will always be printed
    input_verbosity = max(logging.DEBUG, min_log_level - (args.verbose * 10))
    logging.basicConfig(
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s", level=input_verbosity
    )
    del args.verbose

    try:
        r = args.func(args)
        sys.exit(r)
    except Exception as e:
        import traceback

        logger.debug(traceback.format_exc())
        sys.exit(str(e))
