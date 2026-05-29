#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
echo "Service catalog"
curl -fsS "$BASE/system/network/server/list" | python3 -m json.tool | head -n 80 || true
echo "WebDAV status"
curl -fsS "$BASE/system/network/webdav/status"; echo
echo "Enable WebDAV"
curl -fsS "$BASE/system/network/webdav/status?set=enable"; echo
echo "WebDAV OPTIONS"
curl -i -X OPTIONS "$BASE/webdav/" | head -n 20
echo "WebDAV PROPFIND"
curl -i -X PROPFIND -H 'Depth: 1' "$BASE/webdav/" | head -n 40
echo "Native OS status"
for svc in samba ftp sftp webdav; do
  echo "--- $svc"
  curl -fsS "$BASE/system/network/server/osstatus?id=$svc" | python3 -m json.tool | head -n 40 || true
done
