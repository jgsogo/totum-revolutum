# sign with an ad-hoc identity — basically “no real signature, but mark it as signed”.
#
# This is enough to skip "code signature invalid" error.

TARGET_DIR="$(realpath $INSTALL_FOLDER/%TARGET_DIR%)"
log_info "Signing all Mach-O files, .dylib and frameworks under: $TARGET_DIR"

# Helper: sign a file if it looks like Mach-O
sign_if_macho() {
  local f="$1"
  if file "$f" 2>/dev/null | grep -q 'Mach-O'; then
    log_debug "Signing Mach-O: $f"
    codesign --force --sign - --timestamp=none "$f"
  fi
}

# 1) Sign all .dylib files
find "$TARGET_DIR" -type f -name '*.dylib' -print0 |
  while IFS= read -r -d '' lib; do
    sign_if_macho "$lib"
  done

# 2) For each framework, try to find and sign inner binary(s), then sign the bundle
find "$TARGET_DIR" -type d -name '*.framework' -print0 |
  while IFS= read -r -d '' fw; do
    log_debug "Framework: $fw"
    base="$(basename "$fw" .framework)"

    # common binary locations
    candidates=(
      "$fw/Versions/A/$base"
      "$fw/Versions/Current/$base"
      "$fw/$base"
    )

    bin_found=""
    for c in "${candidates[@]}"; do
      if [ -f "$c" ]; then
        bin_found="$c"
        break
      fi
    done

    # fallback: find first Mach-O file under the framework
    if [ -z "$bin_found" ]; then
      bin_found=$(find "$fw" -type f -print0 | while IFS= read -r -d '' f; do
        if file "$f" 2>/dev/null | grep -q 'Mach-O'; then
          printf '%s' "$f"
          break
        fi
      done)
    fi

    if [ -n "$bin_found" ]; then
      sign_if_macho "$bin_found"
    else
      log_debug "  ⚠️  No inner Mach-O binary found for $fw (skipping inner sign)"
    fi

    # attempt signing the framework root as a bundle (best-effort)
    log_debug "Signing framework root: $fw"
    codesign --force --sign - --timestamp=none "$fw" || {
      log_debug "  ⚠️  signing framework root failed for $fw (this can be normal if framework layout is nonstandard)"
    }
  done

# 3) Sign any other Mach-O executables (optional)
find "$TARGET_DIR" -type f -print0 |
  while IFS= read -r -d '' f; do
    # skip already-handled dylibs/framework binaries via file check
    if file "$f" 2>/dev/null | grep -q 'Mach-O'; then
      # Avoid re-signing frameworks' binary twice is OK; codesign will overwrite
      sign_if_macho "$f" || true
    fi
  done
