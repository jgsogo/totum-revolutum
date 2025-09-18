import logging
import pathlib
import re
from dataclasses import dataclass

from jinja2 import Environment, PackageLoader, select_autoescape

log = logging.getLogger(__name__)

RE_FRAMEWORK_LIB = re.compile(r"-framework\s+(\w+)")
RE_TO_SNAKE_CASE = re.compile(r"(?<!^)(?=[A-Z])")


def qt_name(camel_case: str) -> str:
    """Convert camel case name into snake_case"""
    # Convert some strings first so they work property with the snake_case conversion
    camel_case = camel_case.replace("IOS", "Ios")
    return RE_TO_SNAKE_CASE.sub("_", camel_case).lower()


@dataclass
class Framework:
    target: str
    hdrs_dir: str
    lib_files: list[str]
    deps: list[str]

    @property
    def name(self):
        return qt_name(self.target)


def macos_frameworks(lib_path: pathlib.Path):
    for pth in lib_path.glob("*.framework/Resources/*.prl"):
        log.debug("Inspect framework %s", pth)
        with open(pth, "r") as f:
            target = None
            deps = []
            for line in f.readlines():
                key, value = line.split("=")
                key, value = key.strip(), value.strip()
                if key == "QMAKE_PRL_TARGET":
                    target = value
                elif key == "QMAKE_PRL_LIBS":
                    deps = [it for it in RE_FRAMEWORK_LIB.findall(value) if it.startswith("Qt")]

            assert pth.stem == target

            yield Framework(
                target=target,
                deps=deps,
                hdrs_dir=f"{target}.framework/Headers",
                lib_files=[
                    f"{target}.framework/{target}",
                    f"{target}.framework/Versions/A/{target}",
                    f"{target}.framework/Versions/Current/{target}",
                ],
            )


def render(ctx):
    def snake_case(value):
        return qt_name(value)

    env = Environment(loader=PackageLoader("tools.deps"), autoescape=select_autoescape())
    env.filters["snake_case"] = snake_case

    template = env.get_template("qt.BUILD.tpl")
    return template.render(**ctx)
