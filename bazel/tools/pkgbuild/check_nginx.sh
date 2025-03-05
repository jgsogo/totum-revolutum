
# Check if nginx is running
if ! brew services list | grep -c -i "nginx" > /dev/null; then
    log_error "nginx is not running. Start it before installation."
fi
