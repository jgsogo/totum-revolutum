Finances
========

This application is (WIP) migrating to Rust an existing Django application
to manage personal finances. **Until everything is migrated** the database is
managed by the Python application and Rust will be mostly read-only.

## Developer docs

### Dev environment

The following command starts the application from the workspace:

```sh
bazel run //apps/finances/finances-app:dev
```

> **Note.-** See [this issue](https://github.com/jgsogo/totum-revolutum/issues/508), sometimes
> it doesn't work and this command needs to be run first:

> ```sh
> bazel run //apps/finances/finances-app:devserver
> ```

It is also possible to execute the dev environment using non-Bazel tooling:

```sh
bazel run -- @pnpm --dir $(pwd)/apps/finances/finances-app tauri dev
```

### Create bundle to install the application

Right now, I don't know how to create the application bundle using Bazel ([issue](https://github.com/jgsogo/totum-revolutum/issues/507)),
however, the non-Bazel approach works:

```sh
bazel run -- @pnpm --dir $(pwd)/apps/finances/finances-app tauri build
```
