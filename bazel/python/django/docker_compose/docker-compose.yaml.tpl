services:
  web:
    image: %APP_REPOSITORY%:%APP_IMAGE_TAG%
    expose:
      - %DJANGO_PORT%
    environment:
      # Docs: See interpolation and env-files: https://docs.docker.com/compose/how-tos/environment-variables/variable-interpolation/#env-file
      # Django
      - DEBUG=${DEBUG:-true}
      - SECRET_KEY=${SECRET_KEY}  # Django fails if this is empty, so the user really needs to set it from somewhere else
      - DJANGO_ALLOWED_HOSTS=localhost 127.0.0.1 0.0.0.0 [::1]  # TODO: Sure I can remove some of them
      # Database
      - DJANGO_SQL_ENGINE=django.db.backends.postgresql
      - DJANGO_SQL_DATABASE=${SQL_DATABASE:-hello_django_dev}
      - DJANGO_SQL_USER=${SQL_USER:-hello_django}
      - DJANGO_SQL_PASSWORD=${SQL_PASSWORD:-hello_django}
      - DJANGO_SQL_HOST=db
      - DJANGO_SQL_PORT=5432
      # Django static and media files
      - STATIC_ROOT=/home/%USER%/web/staticfiles/
      - MEDIA_ROOT=/home/%USER%/web/mediafiles/
      # Django create superuser
      - DJANGO_SUPERUSER_PASSWORD=${DJANGO_SUPERUSER_PASSWORD}
      - DJANGO_SUPERUSER_USERNAME=${DJANGO_SUPERUSER_USERNAME}
      - DJANGO_SUPERUSER_EMAIL=${DJANGO_SUPERUSER_EMAIL}
      # Django if we execute the migration legacy DB first (FIXME: Remove, this doesn't belong to all apps)
      - LEGACY_DATABASE_URL=${LEGACY_DATABASE_URL}
    depends_on:
      - db
    volumes:
      - static_volume:/home/%USER%/web/staticfiles
      - media_volume:/home/%USER%/web/mediafiles
    healthcheck:
        test: ["CMD", "curl", "-f", "http://localhost:%DJANGO_PORT%/admin"]
        interval: 10s
        retries: 5
        start_period: 60s
        timeout: 10s

  db:
    image: postgres:%POSTGRES_IMAGE_TAG%
    volumes:
      - postgres_data:/var/lib/postgresql/data/
    environment:
      - POSTGRES_USER=${SQL_USER:-hello_django}
      - POSTGRES_PASSWORD=${SQL_PASSWORD:-hello_django}
      - POSTGRES_DB=${SQL_DATABASE:-hello_django_dev}
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U $$POSTGRES_USER -d $$POSTGRES_DB"]
      interval: 10s
      retries: 5
      start_period: 30s
      timeout: 10s

  nginx:
    image: ghcr.io/jgsogo/nginx_django:%NGINX_DJANGO_TAG%
    ports:
      - 1337:80
    depends_on:
      - web
    volumes:
      - static_volume:/home/%USER%/web/staticfiles
      - media_volume:/home/%USER%/web/mediafiles

volumes:
  postgres_data:
  static_volume:
  media_volume:
