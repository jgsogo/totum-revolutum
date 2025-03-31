The only reason for this library is to make the protos from `googleapis` available to other libraries.

## FIXME

In theory, we could generate the javascript targets as follows (see [finances app models](../../apps/finances/tauri/models/)):

```
proto_library(
    name = "protos",
    srcs = [...],
    deps = [
        "@com_google_protobuf//:timestamp_proto",
        "@googleapis//google/type:date_proto",
        "@googleapis//google/type:money_proto",
        "@googleapis//google/type:decimal_proto",
        ],
)

# This rule generates (and copies to the workspace) the proto files for Typescript
ts_proto_library(
    name = "protos_ts",
    node_modules = "//apps/finances/tauri/models:node_modules",
    proto = ":protos",
    proto_srcs = protos,
    protoc_gen_options = {
        "import_extension": "js",
    },
    visibility = ["//visibility:public"],
)
```

and then just consume the `ts_proto_library`:

```
ts_project(
    name = "my_library",
    ...
    deps = [
        ":node_modules",
        "//apps/finances/tauri/models/protos:protos_ts",
    ],
)
```

but the generated `js` files contain relative paths to the `googleapis/google/type/...` protos:

```
import type { Date } from "../../../../../google/type/date_pb.js";
```

and these protos have not been generated... and there is no JS library that provides them (and, anyway, the generated path is wrong).

## Candidate solution

According to [this issue](https://github.com/bufbuild/protobuf-es/issues/457), there is a
`--include-imports` flag to generate the protos for the imported ones as well, but I don't
know how to pass that option to the `ts_proto_library` target.


## Workaround

Copy the _external_ protos I need from [`googleapis`](https://github.com/googleapis/googleapis)
and generate a library that can be reused.
