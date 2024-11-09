Finances
========

This application is (WIP) migrating to Rust an existing Django application
to manage personal finances. **Until everything is migrated** the database is
managed by the Python application and Rust will be mostly read-only.

At this moment there are two applications here:
 * `./tauri/` contains a Tauri application, mostly focus on the app frontend
 * `./django/` contains a Django application, just the admin interface

## Developer docs

### Tauri - Dev environment

The following command starts the application from the workspace:

```sh
bazel run //apps/finances/tauri:dev
```

> **Note.-** See [this issue](https://github.com/jgsogo/totum-revolutum/issues/508), sometimes
> it doesn't work and this command needs to be run first:

> ```sh
> bazel run //apps/finances/tauri:devserver
> ```

It is also possible to execute the dev environment using non-Bazel tooling:

```sh
bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri tauri dev
```

### Tauri - Create bundle to install the application

Right now, I don't know how to create the application bundle using Bazel ([issue](https://github.com/jgsogo/totum-revolutum/issues/507)),
however, the non-Bazel approach works:

```sh
bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri tauri build
```

### Django - Dev environment

To run the development server, use:

```sh
bazel run //apps/finances/django:runserver
```


Django-admin can be executed with the following command:

```sh
bazel run //bazel/python/django/app:admin -- runserver --pythonpath=$(pwd)/apps/finances/django/ --settings=finances.settings
```
