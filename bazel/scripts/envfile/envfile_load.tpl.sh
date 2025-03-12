
# Load the envvars in a file
ENVFILE="$(realpath $INSTALL_FOLDER/%ENVFILE%)"
log_info "Load environment from '$ENVFILE' file"
set -a
source ${ENVFILE}
set +a
