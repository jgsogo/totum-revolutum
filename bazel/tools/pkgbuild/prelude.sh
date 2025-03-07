#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

# Add common paths explicitly
export PATH="$PATH:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

INSTALLER_NAME="%NAME%"
LOGFILE="/var/log/%NAME%_install.log"
LOGLEVEL='%LOG_LEVEL%'

# Captures all stdout and stderr, prepends a timestamp and appends it to the logfile (and to the console)
log_with_timestamp() {
    local log_file="$1"

    # exec > >(while read line; do echo "[$(date '+%Y-%m-%d %H:%M:%S')] $line"; done | tee -a "$logfile") \
    #      2>&1
    exec > >(while read line; do echo "[$(date '+%Y-%m-%d %H:%M:%S')] $line"; done >> "$log_file") \
         2>&1
}

log_with_timestamp $LOGFILE


# Logging functions
function log_output {
  echo "$1"
}

function log_debug {
  if [[ "$LOGLEVEL" =~ ^(DEBUG)$ ]]; then
    log_output "DEBUG $1"
  fi
}

function log_info {
  if [[ "$LOGLEVEL" =~ ^(DEBUG|INFO)$ ]]; then
    log_output "INFO $1"
  fi
}

function log_warn {
  if [[ "$LOGLEVEL" =~ ^(DEBUG|INFO|WARN)$ ]]; then
    log_output "WARN $1"
  fi
}

function log_error {
  if [[ "$LOGLEVEL" =~ ^(DEBUG|INFO|WARN|ERROR)$ ]]; then
    log_output "ERROR $1"
    osascript -e "display dialog \"ERROR: $1\" with title \"$INSTALLER_NAME Installer\""
  fi
}


log_debug "Execute install script '$1'"
