
# Untar some files

for filepath in %FILES_TO_WORK_ON%; do
    FILE_TO_WORK_ON="$(realpath $INSTALL_FOLDER/$filepath)"
    FILE_FOLDER="$(dirname $FILE_TO_WORK_ON)"
    TEMP_DIR="$(mktemp -d)"
    log_info "Untar file '$FILE_TO_WORK_ON' to '$TEMP_DIR'"

    log_info "$(ls -la $FILE_FOLDER/..)"

    tar -xvf "$FILE_TO_WORK_ON" -C "$TEMP_DIR" 2>&1 | LogMsg
    rm -f "$FILE_TO_WORK_ON" 2>&1 | LogMsg
    cp -rf "$TEMP_DIR/*" "$FILE_FOLDER"
    # tar -xvf "$FILE_TO_WORK_ON" -C "$FILE_FOLDER" 2>&1 | LogMsg
done
