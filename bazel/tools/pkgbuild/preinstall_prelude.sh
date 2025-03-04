#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

# Add common paths explicitly
export PATH="$PATH:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

LOG_FILE="/var/log/%NAME%_install.log"

function error() {
  echo >&2 "$(date) installer.bash ERROR: $@" >> "$LOG_FILE"
  osascript -e "display dialog \"Error: $@\" with title \"%NAME% Installer\""
  exit 1
}

echo "Starting pre-install checks..." >> "$LOG_FILE"

## CHUNKS
