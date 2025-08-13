#!/usr/bin/env bash
set -euo pipefail

ALLOW_PRERELEASE="${ALLOW_PRERELEASE:-0}"   # 0 = prefer stable; 1 = allow prereleases

if [[ "$#" -lt 1 ]]; then
    echo "❌ Usage: $0 MODULE1.bazel [MODULE2.bazel ...]"
    exit 1
fi

# portable sed -i
sedi() {
  if sed --version >/dev/null 2>&1; then
    sed -i -E "$@"
  else
    sed -i '' -E "$@"
  fi
}

# Clone Bazel Central Registry
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT
echo "📥 Cloning Bazel Central Registry..."
git clone --depth=1 https://github.com/bazelbuild/bazel-central-registry.git "$TMP_DIR/bcr" >/dev/null

for MODULE_FILE in "$@"; do
    if [[ ! -f "$MODULE_FILE" ]]; then
        echo "❌ Error: '$MODULE_FILE' not found, skipping."
        continue
    fi

    echo "📄 Updating dependencies in $MODULE_FILE"
    cp "$MODULE_FILE" "${MODULE_FILE}.bak"

    # Extract only lines with both name and version
    deps=$(grep -E 'bazel_dep\(' "$MODULE_FILE" | grep -E 'name *= *"' | grep -E 'version *= *"' || true)
    if [[ -z "${deps}" ]]; then
    echo "ℹ️  No bazel_dep entries with explicit versions found."
    exit 0
    fi

    # Iterate each dependency
    echo "$deps" | sed -E 's/.*name *= *"([^"]+)".*version *= *"([^"]+)".*/\1 \2/' |
    while read -r name current_version; do
    [[ -z "$name" || -z "$current_version" ]] && continue
    echo "🔎 Checking ${name} (current: ${current_version})..."

    metadata_path="$TMP_DIR/bcr/modules/$name/metadata.json"
    if [[ ! -f "$metadata_path" ]]; then
        echo "  ⚠️  No metadata found for $name"
        continue
    fi

    all_versions=$(jq -r '.versions[]?' "$metadata_path" 2>/dev/null || true)
    if [[ -z "$all_versions" ]]; then
        echo "  ⚠️  No versions listed for ${name} — skipping"
        continue
    fi

    # Sort versions semver-like, allowing -rc/-beta/etc.
    if [[ "$ALLOW_PRERELEASE" == "1" ]]; then
        latest=$(printf '%s\n' "$all_versions" | sort -V | tail -n1)
    else
        stable=$(printf '%s\n' "$all_versions" | grep -E '^[0-9]+\.[0-9]+\.[0-9]+$' || true)
        if [[ -n "$stable" ]]; then
        latest=$(printf '%s\n' "$stable" | sort -V | tail -n1)
        else
        latest=$(printf '%s\n' "$all_versions" | sort -V | tail -n1)
        fi
    fi

    if [[ -z "$latest" || "$latest" == "null" ]]; then
        echo "  ⚠️  Could not determine latest version for ${name} — skipping"
        continue
    fi

    if [[ "$latest" == "$current_version" ]]; then
        echo "  ✓ Already latest"
        continue
    fi

    echo "  ➜ Updating to ${latest}"

    # BSD/GNU sed-compatible replacement
    # We escape the name in case it has regex special chars
    esc_name=$(printf '%s\n' "$name" | sed -E 's/[][\.^$*+?(){}|/]/\\&/g')
    esc_cur=$(printf '%s\n' "$current_version" | sed -E 's/[][\.^$*+?(){}|/]/\\&/g')
    esc_new=$(printf '%s\n' "$latest" | sed -E 's/[&]/\\&/g')

    sedi "s/(bazel_dep\\([^)]*name *= *\"${esc_name}\"[^)]*version *= *\")${esc_cur}(\"[^)]*\\))/\\1${esc_new}\\2/" "$MODULE_FILE"
    done

    echo "✅ Done updating $MODULE_FILE. Backup saved as ${MODULE_FILE}.bak"

done

echo "🗑️ Remove ${TMP_DIR} directory"
rm -fr $TMP_DIR
echo "💡 Next: run 'bazel mod tidy' to refresh the lockfiles."
