#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"

echo "Testing module list"
curl -fsS "$BASE/system/modules/list" | head -c 300; echo

echo "Testing launch params"
for m in Memo Photo Music NotepadA Camera "Code Studio" "Web Downloader"; do
  echo "--- $m"
  curl -fsS "$BASE/system/modules/getLaunchPara?module=$(python3 - <<PY
import urllib.parse
print(urllib.parse.quote('''$m'''))
PY
)" | head -c 250; echo
 done

echo "Testing static app pages"
for p in Memo/index.html Photo/index.html Music/index.html Camera/index.html Code%20Studio/index.html Web%20Downloader/index.html; do
  echo "--- /$p"
  code=$(curl -sS -o /tmp/arozos_page_test.html -w '%{http_code}' "$BASE/$p")
  echo "HTTP $code bytes $(wc -c </tmp/arozos_page_test.html)"
 done

echo "Testing native app backends"
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Photo/backend/listRoots.js"; echo
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Memo/backend/listmemo.js"; echo
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Music/functions/listSong.js"; echo
