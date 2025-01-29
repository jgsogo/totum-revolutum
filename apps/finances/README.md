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

Use workaround provided by the following command, it setups everything required

```
bazel run //apps/finances:dev
```

The Tauri frontend will start, and Django app is running as well in http://localhost:1337/admin/

---

Backend and frontend can also be started using their corresponding targets:

> **Note.-** You need to execute the frontend first, and then the backend, so the initial connection
  is stablished. Afterwards you can kill and run the frontend and it will work (see watch mode with ibazel).

 * For the **fronted** execute:

   ```sh
   bazel run //apps/finances:frontend
   ```

   You can also execute [this target in watch mode](https://github.com/aspect-build/rules_js/blob/main/docs/js_run_devserver.md)
   using ibazel (this tool monitors the files in the `data` attribute and re-run the target
   if they are modified):

   ```sh
   ibazel run //apps/finances:frontend
   ```

 * For the **backend** execute

   ```sh
   bazel run //apps/finances:backend
   ```

   This target executes a docker compose providing all the underlying infrastructure
   that is needed for this application to run (database, Django admin,...) and then
   executes the Tauri backend connecting to the DB inside docker compose.


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
bazel run //apps/finances/django:app-admin -- runserver
```

Note, that you can use this target to run other Django admin commands:

```
bazel run //apps/finances/django:app-admin -- migrate
DJANGO_SUPERUSER_PASSWORD=django bazel run //apps/finances/django:app-admin -- createsuperuser --email=superuser@app.com --username=django --noinput
```
