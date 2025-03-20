
log_debug "Append host '%HOST%' to /etc/hosts file"

LINE="%HOST%"
grep -qxF "$LINE" /etc/hosts || printf "\n%s\n" "$LINE" >> /etc/hosts
