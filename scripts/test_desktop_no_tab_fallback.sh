#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DESKTOP="$ROOT/resources/web/desktop.html"

echo "Checking desktop launcher does not force browser-tab fallback"
if grep -n 'window.open(normalizeLaunchURL(url)' "$DESKTOP"; then
  echo "ERROR: Rust module launcher still contains direct window.open fallback" >&2
  exit 1
fi

grep -n 'rustCreateEmergencyFloatWindow' "$DESKTOP" >/dev/null
grep -n 'Unable to focus floatWindow iframe' "$DESKTOP" >/dev/null

echo "OK: apps should stay inside ArozOS float windows"
