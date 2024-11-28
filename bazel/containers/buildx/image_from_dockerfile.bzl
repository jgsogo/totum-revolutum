"""A rule to generate an OCI image from a dockerfile"""

load("@aspect_bazel_lib//lib:copy_to_directory.bzl", "copy_to_directory")
load("@aspect_bazel_lib//lib:run_binary.bzl", "run_binary")
load("@configure_buildx//:defs.bzl", "BUILDER_NAME", "TARGET_COMPATIBLE_WITH")
load("@rules_oci//oci:defs.bzl", "oci_load")

def image_from_dockerfile(name, srcs, repository, loadable = False, **kwargs):
    """Generates an OCI image from a Dockerfile

    Args:
        name(str): Name of the generated image
        srcs(List[str]): List of sources (it should include the Dockerfile)
        repository(str): Repository
        loadable(bool): If the `oci_load` target should be created
        **kwargs(dict): Arguments common to all the generated targets (tags, visibility,...)

    This rule generates the following targets:
     * <name>: An OCI image that can be reused in other rules_oci rules
     * <name>-load: (Only if `loadable`) Execute this rule to load the image into the docker registry
     * <name>-tarball: (Only if `loadable`) A tarball with the OCI image. Use it to load the image into a registry at runtime
    """
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

    if loadable:
        native.genrule(
            name = "{}-repo_tags".format(name),
            outs = ["repo_tags.txt"],
            cmd_bash = """
                echo "{}:$$(cat $(location //:stable_build_scm_revision))" > $@
            """.format(repository),
            srcs = ["//:stable_build_scm_revision"],
        )

        oci_load(
            name = "{}-load".format(name),
            image = ":{}".format(name),
            repo_tags = ":{}-repo_tags".format(name),
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
