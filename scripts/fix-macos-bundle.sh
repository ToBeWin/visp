#!/bin/zsh
set -euo pipefail

APP_PATH="${1:-src-tauri/target/release/bundle/macos/Visp.app}"
BUNDLE_ID="${2:-live.visp.translator}"

if [[ ! -d "$APP_PATH" ]]; then
  echo "App bundle not found: $APP_PATH" >&2
  exit 1
fi

codesign --force --deep --sign - --identifier "$BUNDLE_ID" "$APP_PATH"
echo "Re-signed $APP_PATH with identifier $BUNDLE_ID"
