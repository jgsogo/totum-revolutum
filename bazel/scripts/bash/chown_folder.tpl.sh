
CHOWN_FOLDER="%FOLDER%"
log_debug "Change owner of '$CHOWN_FOLDER' to '$INSTALLER_USER:admin'"

sudo chown -R $INSTALLER_USER:admin $CHOWN_FOLDER
