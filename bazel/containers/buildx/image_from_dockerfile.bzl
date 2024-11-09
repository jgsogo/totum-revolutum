"""A rule to generate an OCI image from a dockerfile"""

load("@aspect_bazel_lib//lib:copy_to_directory.bzl", "copy_to_directory")
load("@aspect_bazel_lib//lib:run_binary.bzl", "run_binary")
load("@configure_buildx//:defs.bzl", "BUILDER_NAME", "TARGET_COMPATIBLE_WITH")
load("@rules_oci//oci:defs.bzl", "oci_load")

def image_from_dockerfile(name, srcs, image_tag = None, **kwargs):
    """Generates an OCI image from a Dockerfile

    Args:
        name(str): Name of the generated image
        srcs(List[str]): List of sources (it should include the Dockerfile)
        image_tag(str): The tag to use for the generated image if it's loaded into the docker registry (defaults to "bazel-latest")
        **kwargs(dict): Arguments common to all the generated targets (tags, visibility,...)

    This rule generates the following targets:
     * <name>: An OCI image that can be reused in other rules_oci rules
     * <name>-load: Execute this rule to load the image into the docker registry
     * <name>-tarball: A tarball with the OCI image. Use it to load the image into a registry at runtime
    """
    image_tag = image_tag or "bazel-latest"

    copy_to_directory(
        name = "{}-docker-files".format(name),
        srcs = srcs,
    )

    run_binary(
        name = name,
        srcs = [":{}-docker-files".format(name)],
        args = [
            "build",
            "$(BINDIR)/{}/{}-docker-files".format(native.package_name(), name),
            "--builder",
            BUILDER_NAME,
            "--output=type=oci,tar=false,dest=$@",
        ],
        execution_requirements = {"local": "1"},
        mnemonic = "BuildDocker",
        out_dirs = [name],
        target_compatible_with = TARGET_COMPATIBLE_WITH,
        tool = "//bazel/containers/buildx",
        **kwargs
    )

    oci_load(
        name = "{}-load".format(name),
        image = ":{}".format(name),
        repo_tags = ["{}:{}".format(name, image_tag)],
        **kwargs
    )

    # We need this rule so 'oci_load' actually creates the tarball,
    # see https://github.com/bazel-contrib/rules_oci/blob/main/docs/load.md#build-outputs
    # for more info
    native.filegroup(
        name = "{}-tarball".format(name),
        srcs = [":{}-load".format(name)],
        output_group = "tarball",
        **kwargs
    )
