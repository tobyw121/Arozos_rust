#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
for path in "desktop.html" "Photo/index.html" "Music/index.html" "Memo/index.html" "NotepadA/index.html" "Code%20Studio/index.html" "Web%20Downloader/index.html"; do
  code=$(curl -s -o /dev/null -w '%{http_code}' "$BASE/$path")
  echo "$code $path"
  test "$code" = "200"
done
