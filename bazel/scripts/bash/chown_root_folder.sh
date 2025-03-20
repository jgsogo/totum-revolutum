log_debug "Change owner of '$ROOT_FOLDER' to '$INSTALLER_USER:admin'"
sudo chown -R $INSTALLER_USER:admin $ROOT_FOLDER
