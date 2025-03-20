SOURCE="$(realpath $INSTALL_FOLDER/%SOURCE%)"

log_debug "Create symlink: file '%TARGET%' will point to '$SOURCE'"
ln -Fs $SOURCE %TARGET%
