#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

# Add common paths explicitly
export PATH="$PATH:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

INSTALLER_NAME="%NAME%"
INSTALLER_USER=$(stat -f '%Su' $HOME)
LOGFILE="/var/log/%NAME%_install.log"
LOGLEVEL='%LOG_LEVEL%'
