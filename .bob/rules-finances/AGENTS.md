# Finances App Mode Rules

This file contains app-specific patterns and constraints for the finances application.

## Application Architecture

- **Multi-technology stack**: Qt (C++), Tauri (Rust), Django (Python) - all sharing the same PostgreSQL database
- **Migration in progress**: Migrating from Django to Rust/Tauri - Django manages DB schema, Rust is mostly read-only
- **Three frontends**: Qt desktop app, Tauri desktop app, Django admin interface
- **Shared database**: All three apps connect to the same PostgreSQL database managed by Django migrations

## Development Workflow

- **Primary dev target**: `bazel run //apps/finances:dev` - starts Tauri frontend + Django backend with docker-compose
- **Frontend only**: `bazel run //apps/finances:frontend` (supports ibazel watch mode)
- **Backend only**: `bazel run //apps/finances:backend` (docker-compose + Tauri backend)
- **Django admin**: `bazel run //apps/finances/django:app-admin -- runserver`
- **Qt dev**: `just finances cpp-dev` or `bazel run //apps/finances/qt:dev`

## Database Management (Critical)

- **Django owns schema**: All database migrations MUST be done through Django (`//apps/finances/django:app-admin -- migrate`)
- **Rust is read-only**: Until migration is complete, Rust/Tauri should not modify database schema
- **Diesel schema sync**: After Django migrations, regenerate Diesel schema with `bazel run //apps/finances/tauri/src-tauri:<schema>.update`
- **Postgres tests**: Use `with_postgres_test()` wrapper for tests requiring database

## Build Patterns (App-Specific)

- **Tauri debug assertions**: Always enabled (inherited from monorepo pattern) - required by Tauri macros
- **Bundle creation**: Currently not working through Bazel - use `bazel run -- @pnpm --dir $(pwd)/apps/finances/tauri tauri build`
- **Release builds**: Use `--config=release --config=stamp` for versioned releases
- **Installable package**: `bazel build --config=release --config=stamp //apps/finances/installable`

## Testing Patterns

- **Integration tests**: Must be in `tests/test_*.rs` (flat structure)
- **Shared test helpers**: Use `shared_srcs` in `rust_test_suite`
- **Django tests**: Standard Django test patterns apply
- **Database fixtures**: Django manages test database setup

## Release Management

- **Versioning**: Uses date-based tags with prefix `finances-` (e.g., `finances-2024-01-20`)
- **Release workflow**: `just finances release` - runs tests, creates tag, builds installable, uploads to GitHub
- **Installation**: `just finances install` - downloads and installs latest release from GitHub

## Technology-Specific Notes

### Tauri (Rust)
- Located in `apps/finances/tauri/`
- Uses Diesel ORM for database access
- Frontend: Svelte + TypeScript
- Backend: Rust with Tauri framework

### Django (Python)
- Located in `apps/finances/django/`
- Manages database schema and migrations
- Provides admin interface at http://localhost:1337/admin/
- Uses docker-compose for local development

### Qt (C++)
- Located in `apps/finances/qt/`
- Desktop application using Qt framework
- Direct PostgreSQL connection via libpqxx
- Separate from Tauri/Django but shares database

## Common Gotchas

- **Frontend before backend**: When starting separately, run frontend first to establish initial connection
- **Django superuser**: Create with `DJANGO_SUPERUSER_PASSWORD=django bazel run //apps/finances/django:app-admin -- createsuperuser --email=superuser@app.com --username=django --noinput`
- **Docker compose**: Backend target automatically manages docker-compose lifecycle
- **Schema sync**: Always regenerate Diesel schema after Django migrations
