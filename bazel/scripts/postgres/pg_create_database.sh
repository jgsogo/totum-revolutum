# TODO: In the rule we can request the mapping from whatever are the envvars I have to the standard 'DB_xxx' ones
DB_ROOT_USER=$INSTALLER_USER

# Function to check database connectivity
check_db_connection() {
    log_debug 'check_db_connection'
    PGPASSWORD="$DJANGO_SQL_PASSWORD" psql -h "$DJANGO_SQL_HOST" -p "$DJANGO_SQL_PORT" -U "$DJANGO_SQL_USER" -lqt &>/dev/null
    return $?
}

# Function to check if the database exists
check_database_exists() {
    log_debug 'check_database_exists'
    PGPASSWORD="$DJANGO_SQL_PASSWORD" psql -h "$DJANGO_SQL_HOST" -p "$DJANGO_SQL_PORT" -U "$DJANGO_SQL_USER" -tc "SELECT 1 FROM pg_database WHERE datname='$DJANGO_SQL_DATABASE'" | grep -q 1
    return $?
}

# Function to create the database if it doesn’t exist
create_database() {
    log_debug "create_database using user '$DB_ROOT_USER', on dbname=postgres"
    # These command run with whatever is the _default_ user, it should have enough permissions to create the database, we are also creating the django user and assigning it to the new database
    psql -h "$DJANGO_SQL_HOST" -p "$DJANGO_SQL_PORT" -U "$DB_ROOT_USER" --dbname=postgres -c "CREATE DATABASE $DJANGO_SQL_DATABASE;"
    psql -h "$DJANGO_SQL_HOST" -p "$DJANGO_SQL_PORT" -U "$DB_ROOT_USER" --dbname=postgres -c "CREATE USER $DJANGO_SQL_USER WITH ENCRYPTED PASSWORD '$DJANGO_SQL_PASSWORD';" || true  # User may already exist
    psql -h "$DJANGO_SQL_HOST" -p "$DJANGO_SQL_PORT" -U "$DB_ROOT_USER" --dbname=postgres -c "GRANT ALL PRIVILEGES ON DATABASE $DJANGO_SQL_DATABASE TO $DJANGO_SQL_USER;"
}

if ! check_db_connection; then
    log_error "Cannot connect to database with the given credentials: host '$DJANGO_SQL_HOST', port '$DJANGO_SQL_PORT', user '$DJANGO_SQL_USER'"
    exit 1
fi

# Check if the database exists, create it if not
if ! check_database_exists; then
    create_database
    if check_database_exists; then
        osascript -e "Tell application \"System Events\" to display alert \"Database Created\" message \"Database '$DJANGO_SQL_DATABASE' was successfully created.\""
    else
        osascript -e "Tell application \"System Events\" to display alert \"Database Creation Failed\" message \"Could not create database '$DJANGO_SQL_DATABASE'. Please check permissions.\""
        exit 1
    fi
fi
