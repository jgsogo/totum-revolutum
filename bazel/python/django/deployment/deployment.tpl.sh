#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

FORCE=
POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
  case $1 in
    --force)
      FORCE=1
      shift # past argument
      ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1
      ;;
    *)
      POSITIONAL_ARGS+=("$1") # save positional arg
      shift # past argument
      ;;
  esac
done

set -- "${POSITIONAL_ARGS[@]}" # restore positional parameters

OUTPUT_DIRECTORY=$POSITIONAL_ARGS

if [ ! -d "$OUTPUT_DIRECTORY" ]; then
  echo "Directory '$OUTPUT_DIRECTORY' does not exist."
  exit 1
fi

echo "Deploy to '$OUTPUT_DIRECTORY'"

# Environment file
ENV_FILE=$OUTPUT_DIRECTORY/.env
if [ -f $ENV_FILE ] && [ -z "$FORCE" ]; then
    echo "File '$ENV_FILE' already exists. Use '--force' to override!"
fi

echo "# Django" > $ENV_FILE
echo "DEBUG=False" >> $ENV_FILE
echo "SECRET_KEY=<secret-key>" >> $ENV_FILE
echo "DJANGO_ALLOWED_HOSTS=\"localhost 127.0.0.1 0.0.0.0 [::1]\"" >> $ENV_FILE
echo "" >> $ENV_FILE

echo "# Database" >> $ENV_FILE
echo "DJANGO_SQL_ENGINE=" >> $ENV_FILE
echo "DJANGO_SQL_DATABASE=" >> $ENV_FILE
echo "DJANGO_SQL_USER=" >> $ENV_FILE
echo "DJANGO_SQL_PASSWORD=" >> $ENV_FILE
echo "DJANGO_SQL_HOST=" >> $ENV_FILE
echo "DJANGO_SQL_PORT=" >> $ENV_FILE
echo "" >> $ENV_FILE

echo "# Django static and media files" >> $ENV_FILE
STATIC_ROOT=$OUTPUT_DIRECTORY/static/
echo "STATIC_ROOT=$STATIC_ROOT" >> $ENV_FILE
MEDIA_ROOT=$OUTPUT_DIRECTORY/media/
echo "MEDIA_ROOT=$MEDIA_ROOT" >> $ENV_FILE
echo "" >> $ENV_FILE

echo "# gunicorn" >> $ENV_FILE
echo "GUNICORN_APP_NAME=%NAME%" >> $ENV_FILE
TODO: We are removing all this file, right!?
echo "GUNICORN_WORKING_DIR=$OUTPUT_DIRECTORY" >> $ENV_FILE
echo "GUNICORN_USER=$(whoami)" >> $ENV_FILE
echo "GUNICORN_GROUP=$(id -gn jgsogo)" >> $ENV_FILE
SOCKET="unix:$OUTPUT_DIRECTORY/run/gunicorn.sock"
echo "GUNICORN_BIND=$SOCKET" >> $ENV_FILE

# Supervisor file
SUPERVISOR_FILE=$OUTPUT_DIRECTORY/supervisord.ini
if [ -f $SUPERVISOR_FILE ] && [ -z "$FORCE" ]; then
    echo "File '$SUPERVISOR_FILE' already exists. Use '--force' to override!"
fi

echo "[program:%NAME%]" > $SUPERVISOR_FILE
echo "command=export \$(grep -v '^#' $ENV_FILE | xargs -d '\n') && $OUTPUT_DIRECTORY/%GUNICORN_APP%" >> $SUPERVISOR_FILE
echo "stdout_logfile=$OUTPUT_DIRECTORY/log/supervisord.stdout.log" >> $SUPERVISOR_FILE
echo "stdout_logfile_maxbytes=1MB" >> $SUPERVISOR_FILE
echo "stdout_logfile_backups=10" >> $SUPERVISOR_FILE
echo "stdout_capture_maxbytes=1MB" >> $SUPERVISOR_FILE
echo "stdout_events_enabled=false" >> $SUPERVISOR_FILE
echo "stderr_logfile=$OUTPUT_DIRECTORY/log/supervisord.stderr.log" >> $SUPERVISOR_FILE
echo "stderr_logfile_maxbytes=1MB" >> $SUPERVISOR_FILE
echo "stderr_logfile_backups=10" >> $SUPERVISOR_FILE
echo "stderr_capture_maxbytes=1MB" >> $SUPERVISOR_FILE
echo "stderr_events_enabled=false" >> $SUPERVISOR_FILE


# Nginx file
NGINX_FILE=$OUTPUT_DIRECTORY/nginx.conf
if [ -f $NGINX_FILE ] && [ -z "$FORCE" ]; then
    echo "File '$NGINX_FILE' already exists. Use '--force' to override!"
fi

echo "upstream %NAME%_server {" > $NGINX_FILE
echo "  server $SOCKET fail_timeout=0;" >> $NGINX_FILE
echo "}" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "server {" >> $NGINX_FILE
echo "  listen 80;" >> $NGINX_FILE
echo "  server_name localhost.%NAME%;" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "  large_client_header_buffers 4 32k;" >> $NGINX_FILE
echo "  client_max_body_size 50M;" >> $NGINX_FILE
echo "  charset utf-8;" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "  access_log $OUTPUT_DIRECTORY/log/nginx.access.log;" >> $NGINX_FILE
echo "  error_log $OUTPUT_DIRECTORY/log/nginx.error.log;" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "  location /static/ {" >> $NGINX_FILE
echo "    alias   $STATIC_ROOT;" >> $NGINX_FILE
echo "  }" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "  location /media/ {" >> $NGINX_FILE
echo "    alias   $MEDIA_ROOT;" >> $NGINX_FILE
echo "  }" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "  location / {" >> $NGINX_FILE
echo "    proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;" >> $NGINX_FILE
echo "    proxy_set_header Host \$http_host;" >> $NGINX_FILE
echo "    proxy_redirect off;" >> $NGINX_FILE
echo "    if (!-f \$request_filename) {" >> $NGINX_FILE
echo "      proxy_pass http://%NAME%_server;" >> $NGINX_FILE
echo "      break;" >> $NGINX_FILE
echo "    }" >> $NGINX_FILE
echo "  }" >> $NGINX_FILE
echo "" >> $NGINX_FILE

echo "  error_page 500 502 503 504 /500.html;" >> $NGINX_FILE
echo "  location = /500.html {" >> $NGINX_FILE
echo "    root $STATIC_ROOT;" >> $NGINX_FILE
echo "  }" >> $NGINX_FILE
echo "}" >> $NGINX_FILE
