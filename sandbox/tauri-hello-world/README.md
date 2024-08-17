Tauri application
=================

This example is very fragile. There are many things that are not tested automatically:
 * Devserver is not tested. We may break it with any update
 * Actual artifact generation is also not tested. It requires a custom command (see below)

## Development

```
bazel run --verbose_failures //sandbox/tauri-hello-world:dev
```

This target runs the frontend process and the backend one. The artifacts are built using
Bazel.

## Generate final artifacts

In this `ignore/build-folder` branch I've tried to call `tauri dev` from Bazel, but I was
not able to make it work. Anyway, that's **not the right Bazel approach**. Using Bazel we
already have the binaries generated and we need to execute a rule to compose the final
artifact using those binaries, we don't want to build everything again.

If we keep the project structure and the content of all files, this works to generate
the binaries (of course, it happens outside Bazel):

```
bazel run -- @pnpm --dir $(pwd) tauri build
```
