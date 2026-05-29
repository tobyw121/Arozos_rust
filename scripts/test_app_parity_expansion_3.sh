#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
echo "Testing Photo wildcard listFolder"
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Photo/backend/listFolder.js" -H 'Content-Type: application/json' --data '{"folder":"user:/Photo/*"}' | head -c 400; echo

echo "Testing PDF viewer asset"
curl -fsSI "$BASE/PDF%20Viewer/viewer.html" | head -5

echo "Testing NotepadA legacy PHP alias"
curl -fsSI "$BASE/NotepadA/index.php" | head -5

echo "Testing Memo add/list"
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Memo/backend/addMemo.js" -d 'memo={"title":"Rust parity test","message":"OK","deadline":0}' | head -c 200; echo
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Memo/backend/listmemo.js" | head -c 400; echo

echo "Testing Music wildcard/media cleanup"
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Music/functions/listSong.js" -d 'listdir=user:/Music/*' | head -c 400; echo

echo "Testing Browser title fallback"
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Browser/functions/getTitle.js" -d 'url=https://example.com/' | head -c 200; echo
