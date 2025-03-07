
# Execute a Django command
log_info "Execute Django command '%COMMAND%'"
DJANGO_ADMIN="$BIN_FOLDER/%DJANGO_ADMIN_APP%"
$DJANGO_ADMIN %COMMAND%
