#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
AUTH="${2:-}"
CURL=(curl -fsS)
if [ -n "$AUTH" ]; then CURL+=( -u "$AUTH" ); fi

echo "== WebDAV HTML index =="
"${CURL[@]}" "$BASE/webdav/" | grep -E "ArozOS WebDAV|Download|Native Rust WebDAV" >/dev/null && echo "webdav index ok"

echo "== WebDAV protocol OPTIONS =="
curl -fsSI -X OPTIONS "$BASE/webdav/" | grep -i "dav:" || true

echo "== Service status before/after disable =="
for svc in ftp tftp sftp telnet; do
  echo "-- $svc stop"
  curl -fsS "$BASE/system/storage/$svc/stop" || true
  echo
  curl -fsS "$BASE/system/storage/$svc/status" || true
  echo
 done

echo "-- samba disable"
curl -fsS -X POST "$BASE/system/storage/samba/status" -d 'set=disable' || true
echo
curl -fsS "$BASE/system/storage/samba/status" || true
echo

echo "== Service catalog =="
curl -fsS "$BASE/system/network/server/list" | head -c 2000
echo
