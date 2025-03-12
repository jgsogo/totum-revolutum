
# Combine envfiles
log_info "Combine source '%SOURCE%' with '%TARGET%'"

SOURCE="$(realpath $INSTALL_FOLDER/%SOURCE%)"
TARGET="$INSTALL_FOLDER/%TARGET%"

touch $TARGET
TARGET="$(realpath $TARGET)"

TEMP_FILE="$(mktemp -d)/file.tmp"

# Creates a tem_file with the content of both files (target goes first)
cat $TARGET > $TEMP_FILE
grep "\S" $SOURCE  >> $TEMP_FILE

# Takes the first ocurrence of every key (if a value is already provided)
awk -F "=" '!a[$1]++' $TEMP_FILE > $TARGET
