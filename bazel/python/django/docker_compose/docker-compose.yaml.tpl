name: %APP_NAME%
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
      - SQL_ENGINE=django.db.backends.postgresql
      - SQL_DATABASE=${SQL_DATABASE:-hello_django_dev}
      - SQL_USER=${SQL_USER:-hello_django}
      - SQL_PASSWORD=${SQL_PASSWORD:-hello_django}
      - SQL_HOST=db
      - SQL_PORT=5432
      - DATABASE=postgres
      # Django static and media files
      - STATIC_ROOT=/home/%USER%/web/staticfiles/
      - MEDIA_ROOT=/home/%USER%/web/mediafiles/
    depends_on:
      - db
    volumes:
      - static_volume:/home/%USER%/web/staticfiles
      - media_volume:/home/%USER%/web/mediafiles
    healthcheck:
        test: ["CMD", "curl", "-f", "http://localhost:%DJANGO_PORT%/admin"]
        interval: 10s
        retries: 5
        start_period: 30s
        timeout: 10s

  db:
    image: postgres:17
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
