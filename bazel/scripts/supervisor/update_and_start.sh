
log_debug "Reload and start supervisor application"

sudo supervisorctl reread
sudo supervisorctl update
sudo supervisorctl restart $INSTALLER_NAME
