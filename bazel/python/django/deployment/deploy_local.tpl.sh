#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars


# Template variables:
# Array of paths of files
declare -r -a SOURCE_FILES=@@SOURCE_FILES@@
# Array of paths relative to prefix.
declare -r -a TARGET_NAMES=@@TARGET_NAMES@@
# Array marking the files to unt.
declare -r -a UNTARS=@@UNTARS@@
# Set executable mode of files
# declare -r EXECUTABLE=@@EXECUTABLE@@
# Fully specified bazel label of INSTALLER_LABEL installer() rule.
declare -r INSTALLER_LABEL=@@INSTALLER_LABEL@@
# Number of files.
declare -r -i N_FILES="${#SOURCE_FILES[@]}"


function error() {
  echo >&2 "installer.bash ERROR: $@"
  usage
  exit 1
}

function usage() {
  echo >&2 "Usage: bazel run ${INSTALLER_LABEL} [-c opt] [--] [-s] [--] /INSTALL_PREFIX"
}

# Checks that all template variables have been substituted.
function verify_templates() {
  if [[ "${SOURCE_FILES[@]:0:2}" =~ ^@@ ]] ||
    [[ "${TARGET_NAMES[@]:0:2}" =~ ^@@ ]] ||
    [[ "${UNTARS[@]:0:2}" =~ ^@@ ]] ||
    [[ "${INSTALLER_LABEL:0:2}" =~ ^@@ ]]; then
    error "template substitution failed"
  fi

  if [[ "${#SOURCE_FILES[@]}" != "${#TARGET_NAMES[@]}" ]]; then
    error "the number of source files is different thant the number target names"
  fi

  if [[ "${#SOURCE_FILES[@]}" != "${#UNTARS[@]}" ]]; then
    error "the number of source files is different thant the number or untar marks"
  fi
}

# Checks that each path in $SOURCE_FILES[@] is a readable file.
function check_sources() {
  for source in "${SOURCE_FILES[@]}"; do
    if ! [[ -r "$(rlocation ${source})" ]]; then
      error "Can't read '${source}' located at '$(rlocation ${source})'"
    fi
  done
}

# Installs $i-th file in $prefix.
function install_file() {
  local prefix="$1"
  local i="$2"
  local sudo="$3"
  local source="$(rlocation ${SOURCE_FILES[${i}]})"
  local target="${TARGET_NAMES[${i}]}"
  local untar="${UNTARS[${i}]}"
  local target_dir
  target_dir="$(dirname -- "${prefix}/${target}")"
  local target_name
  target_name="$(basename -- "${target}")"
  local target_mode
  # if [[ "${EXECUTABLE}" = "True" ]]; then
  #   target_mode=755
  # else
  #   target_mode=644
  # fi

  [[ -d "${target_dir}" ]] || \
    $sudo mkdir -p "${target_dir}"

  # $sudo install -m "${target_mode}" \
  #   -T -- "${source}" "${target_dir}/${target_name}"

  echo "Copy file '${target_dir}/${target_name}'"
  $sudo install \
    -T -- "${source}" "${target_dir}/${target_name}"

  if [ -z "${untar}" ]; then
    echo "Untar file '${target_dir}/${target_name}'"
    $sudo tar -xf "${target_dir}/${target_name}" -C "${target_dir}"
    $sudo rm "${target_dir}/${target_name}"
  fi
}


function main() {
  verify_templates

  local g_flag=''
  local s_flag=''
  while getopts ':ghs' flag; do
    case "${flag}" in
      g) g_flag='' ;;  # Ignored - debug builds flag
      s) s_flag='sudo' ;;  # Run with sudo
      h) usage; exit 0 ;;
      *) error "Unexpected option '-${OPTARG}'"   ;;
    esac
  done

  if [[ -z "${OPTIND:-}" ]] || (($OPTIND > $#)); then
    error "INSTALL_PREFIX wasn't specified"
  fi
  local prefix="${!OPTIND}"

  # Convert Windows path to Unix path
  if command -v cygpath &> /dev/null; then
    prefix="$(cygpath $prefix)"
  fi

  if [[ "${prefix:0:1}" != "/" ]]; then
    error "INSTALL_PREFIX must be an absolute path"
  fi

  # check_sources
  local i
  for ((i=0; i<${N_FILES}; i++)); do
    install_file "${prefix}" "${i}" "${s_flag}"
  done
}

main "$@"
