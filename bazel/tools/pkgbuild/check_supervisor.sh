# Check if supervisord is installed
if ! command -v supervisorctl &> /dev/null; then
    error "supervisord is not installed. Please install it first."
fi

# Check if supervisord is running
if ! ps aux | grep -v grep | grep -c -i "supervisor" > /dev/null; then
    error "supervisord is not running. Start it before installation."
fi
