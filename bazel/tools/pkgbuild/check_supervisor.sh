# Check if supervisord is installed
log_debug "Check if supervisorctl is installed"
if ! command -v supervisorctl &> /dev/null; then
    log_error "supervisord is not installed. Please install it first."
fi

# Check if supervisord is running
log_debug "Check if supervisord is running"
if ! ps aux | grep -v grep | grep -c -i "supervisor" > /dev/null; then
    log_error "supervisord is not running. Start it before installation."
fi
