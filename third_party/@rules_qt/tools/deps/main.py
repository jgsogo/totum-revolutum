import argparse
import logging
import pathlib

from tools.deps.macos import macos_frameworks, render

log = logging.getLogger(__name__)


def main(ctx: dict, input_path: pathlib.Path, output: pathlib.Path):
    log.info("Work on directory '%s'", input_path)

    # Mac
    frameworks = list(macos_frameworks(input_path / "lib"))
    ctx.update({"frameworks": frameworks})

    content = render(ctx)
    with open(output, "w") as f:
        f.write(content + "\n")


if __name__ == "__main__":
    logging.basicConfig(
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s", level=logging.DEBUG
    )

    parser = argparse.ArgumentParser(description="Compute dependencies between Qt libraries")
    parser.add_argument(
        "--input_path",
        type=pathlib.Path,
        required=True,
        help="A directory that contains a Qt installation (created by aqt install tool)",
    )
    parser.add_argument("--base_path", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, required=True)

    parser.add_argument("--host", type=str, required=True)
    parser.add_argument("--target_sdk", type=str, required=True)
    parser.add_argument("--version", type=str, required=True)
    parser.add_argument("--arch", type=str, required=True)

    args = parser.parse_args()

    path_prefix = args.input_path.relative_to(args.base_path)
    ctx = {
        "host": args.host,
        "target_sdk": args.target_sdk,
        "version": args.version,
        "arch": args.arch,
        "path_prefix": path_prefix,
    }

    main(ctx, args.input_path, args.output)
