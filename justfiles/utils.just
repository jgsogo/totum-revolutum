cgreen := "\\033[1;32m"
cred := "\\033[0;31m"
creset := "\\033[0m"

_latest-tag prefix latest_tag_filepath:
    @{{ justfile_directory() }}/scripts/gh/latest_git_tag.sh {{ prefix }} >> {{ latest_tag_filepath }}
    @echo "Latest version is '{{ cgreen }}$(cat {{ latest_tag_filepath }}){{ creset }}'"

_next-tag prefix next_tag_filepath:
    @{{ justfile_directory() }}/scripts/gh/next_git_tag.sh {{ prefix }} >> {{ next_tag_filepath }}
    @echo "Next tag is '{{ cgreen }}$(cat {{ next_tag_filepath }}){{ creset }}'"

# Check (and fail) if there are local changes
_gh_check_local_changes:
    #!/usr/bin/env bash
    if [[ `git status --porcelain` ]]; then
        printf "{{ cred }}🛑 STOP:{{ creset }} There are local changes in your Git working directory\n"
        exit 1
    else
        exit 0
    fi

# Check (and fail) if the working copy is not the HEAD of the default branch
_gh_check_i_am_head:
    @{{ justfile_directory() }}/scripts/gh/i_am_default_head.sh

# Run all 'oci_load' targets
_bazel_run_oci_load bazel_config="":
    #!/usr/bin/env bash
    set -euox pipefail

    targets=$(bazel query 'kind("oci_load", //...)')

    while IFS= read -r line; do
        bazel run {{ bazel_config }} $line
    done <<< "$targets"

# Run all targets with the given tag
_bazel_run_targets tag bazel_config="":
    #!/usr/bin/env bash
    set -euox pipefail

    targets=$(bazel query "attr('tags', '{{ tag }}', '//...')")

    while IFS= read -r line; do
        bazel run {{ bazel_config }} $line
    done <<< "$targets"
