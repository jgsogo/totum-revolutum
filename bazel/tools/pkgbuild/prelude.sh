#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

# Add common paths explicitly
export PATH="$PATH:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

INSTALLER_NAME="%NAME%"
LOGFILE="/var/log/%NAME%_install.log"
LOGLEVEL='%LOG_LEVEL%'

# Logging functions
function log_output {
  echo `date "+%Y/%m/%d %H:%M:%S"`" $1"
  echo `date "+%Y/%m/%d %H:%M:%S"`" $1" >> $LOGFILE
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

LogMsg()
{
  read IN # This reads a string from stdin and stores it in a variable called IN
  log_info "$IN"
}

log_debug "Execute install script '$1'"
