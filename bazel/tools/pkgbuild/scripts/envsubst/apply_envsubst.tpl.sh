
# Apply envsubst to some files
log_debug "Apply envsubst to files"

ENVSUBST_TOOL="$BIN_FOLDER/envsubst"
chmod +x $ENVSUBST_TOOL

TEMP_FILE="$(mktemp -d)/file.tmp"
for filepath in %FILES_TO_WORK_ON%; do
    FILE_TO_WORK_ON="$(realpath $INSTALL_FOLDER/$filepath)"

    log_info "Apply envsubst to file '$FILE_TO_WORK_ON'"
    $ENVSUBST_TOOL -no-unset -no-empty < $FILE_TO_WORK_ON > $TEMP_FILE && mv $TEMP_FILE $FILE_TO_WORK_ON
done
