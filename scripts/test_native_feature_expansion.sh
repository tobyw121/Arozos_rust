#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"

echo "== module launch data =="
for m in Photo Music Memo NotepadA "Code Studio" "Web Downloader"; do
  curl -fsS "$BASE/system/modules/getLaunchPara?module=$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1]))' "$m")" | python3 -m json.tool >/dev/null
  echo "$m OK"
done

echo "== app backends =="
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Photo/backend/listRoots.js" | python3 -m json.tool >/dev/null
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Memo/backend/listmemo.js" | python3 -m json.tool >/dev/null
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Music/functions/listSong.js" | python3 -m json.tool >/dev/null
curl -fsS -X POST "$BASE/system/ajgi/interface?script=Browser/functions/bookmark.js" -d 'opr=read' | python3 -m json.tool >/dev/null

echo "== SystemAO compatibility endpoints =="
curl -fsS "$BASE/system/permission/listgroup?showper=true" | python3 -m json.tool >/dev/null
curl -fsS "$BASE/system/disk/quota/quotaInfo" | python3 -m json.tool >/dev/null
curl -fsS "$BASE/system/disk/quota/quotaDist" | python3 -m json.tool >/dev/null
curl -fsS "$BASE/system/disk/quota/listQuota" | python3 -m json.tool >/dev/null
curl -fsS "$BASE/system/backup/snapshotSummary" | python3 -m json.tool >/dev/null

echo "== media helpers =="
curl -fsS "$BASE/media/getMime/?file=user%3A%2FPhoto%2Fmissing.jpg" >/dev/null || true

echo "native feature expansion smoke test completed"
