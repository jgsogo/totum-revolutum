
# Untar some files
log_debug "Untar files"

for filepath in %FILES_TO_WORK_ON%; do
    log_info "Untar file '$filepath'"
    FILE_TO_WORK_ON="$(realpath $INSTALL_FOLDER/$filepath)"
    FILE_FOLDER="$(dirname $FILE_TO_WORK_ON)"
    TEMP_DIR="$(mktemp -d)"

    log_debug " - Untar file '$FILE_TO_WORK_ON' to '$TEMP_DIR'"
    tar -xvf "$FILE_TO_WORK_ON" -C "$TEMP_DIR" 2>&1 | LogMsg
    rm -f "$FILE_TO_WORK_ON"

    log_debug " - Rsync files from '$TEMP_DIR' into '$FILE_FOLDER'"
    rsync -az "$TEMP_DIR/" "$FILE_FOLDER"
done
