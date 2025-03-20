SOURCE="$(realpath $INSTALL_FOLDER/%SOURCE%)"

log_debug "Create symlink: file '%TARGET%' will point to '$SOURCE'"
ln -Fs $SOURCE %TARGET%

DO_CHOWN="%DO_CHOWN%"
if [[ -n "${DO_CHOWN// /}" ]]; then
    log_debug "chown '%TARGET%' to '$INSTALLER_USER:admin'"
    chown -h $INSTALLER_USER:admin %TARGET%  # Use -h so it modifies the symlink itself and not the target file
fi
