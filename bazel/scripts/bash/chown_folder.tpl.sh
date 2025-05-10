
log_debug "Change owner of '%FOLDER%' to '$INSTALLER_USER:admin'"

sudo chown -R $INSTALLER_USER:admin "%FOLDER%"
