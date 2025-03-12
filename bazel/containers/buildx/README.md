buildx
======

The rules in this directory allow us to create OCI containers (for `rules_oci`) from a `Dockerfile`
using [docker-buildx](https://docs.docker.com/reference/cli/docker/buildx/).

> **Note.-** Docker containers are not reproducible, this breaks hermeticity principle from Bazel
> but it's very convenient sometimes. Please read the notes in the `rules_oci` repository for
> more information ([link](https://github.com/bazel-contrib/rules_oci/tree/a195e365b8653139bef7f6bbb3e7b00b4eabd024/examples/dockerfile)).

Here I adapted the [example from `rules_oci`](https://github.com/bazel-contrib/rules_oci/tree/a195e365b8653139bef7f6bbb3e7b00b4eabd024/examples/dockerfile)
to use it via Bazel extensions (otherwise, it doesn't work in Linux).

## Usage:

**MODULE.bazel**

Import the extension and declare all the external repositories required by buildx

```bzl
fetch_buildx = use_extension("//bazel/containers/buildx:extensions.bzl", "fetch_buildx")
use_repo(fetch_buildx, "buildx_darwin_amd64", "buildx_darwin_arm64", "buildx_linux_amd64", "configure_buildx")
```

**BUILD.bazel**

Use the `image_from_dockerfile` macro to create the OCI containers and the targets that
will allow us to use those images in the local docker registry and also in runtime (like `testcontainers`).
Alternatively, you can use directly the rules inside the macro.

```bzl
load("//bazel/containers/buildx:image_from_dockerfile.bzl", "image_from_dockerfile")

image_from_dockerfile(
    name = "nginx",
    srcs = [
        "Dockerfile",
        "nginx.conf",
    ],
    tags = [
        "manual",
        "no-remote-cache",
    ],
    visibility = [
        "//apps:__subpackages__",
    ],
)
```
