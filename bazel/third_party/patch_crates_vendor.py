"""
For some reason, the output of 'bazel run @@//bazel/third_party:crates_vendor'
contains some bugs that we need to fix.
"""

import pathlib
import sys


def remove_cargo_bazel_json(build_file: pathlib.Path):
    with open(build_file, "r") as f:
        lines = f.readlines()
    with open(build_file, "w") as f:
        for line in lines:
            if "cargo-bazel.json" in line:
                continue
            f.write(line)


def remove_alias(build_file: pathlib.Path, alias: str):
    with open(build_file, "r") as f:
        lines = f.readlines()

    alias_content = None
    alias_pattern = f'name = "{alias}"'
    with open(build_file, "w") as f:
        for line in lines:
            if line == "alias(\n":
                assert alias_content is None
                alias_content = []

            if alias_content is not None:
                alias_content.append(line)

                if line == ")\n":
                    # Work on the alias group
                    if any(alias_pattern in alias_line for alias_line in alias_content):
                        pass
                    else:
                        # Dump full alias to a file
                        for alias_line in alias_content:
                            f.write(alias_line)

                    # Clear alias
                    alias_content = None
            else:
                f.write(line)


if __name__ == "__main__":
    crates_directory = pathlib.Path(sys.argv[1])
    build_file = crates_directory / "BUILD.bazel"

    assert build_file.exists(), f"File '{build_file}' doesn't exist"

    remove_cargo_bazel_json(build_file=build_file)

    remove_alias(build_file=build_file, alias="googleapis")
    remove_alias(build_file=build_file, alias="googleapis-0.1.0")

    remove_alias(build_file=build_file, alias="finances-app-0.1.0")
    remove_alias(build_file=build_file, alias="finances-app")
    remove_alias(build_file=build_file, alias="finances-app-models-0.1.0")
    remove_alias(build_file=build_file, alias="finances-app-models")
