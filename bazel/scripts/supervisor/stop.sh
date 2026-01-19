
log_debug "Stop supervisor application (if it exists)"

sudo supervisorctl stop $INSTALLER_NAME || true  # In a fresh install the INSTALLER_NAME doesn't exist yet
