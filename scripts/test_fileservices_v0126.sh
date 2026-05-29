#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"

echo "== network server list =="
curl -fsS "$BASE/system/network/server/list" | python3 -m json.tool | head -n 80 || true

echo "== endpoints =="
curl -fsS "$BASE/system/network/server/endpoints" | python3 -m json.tool

echo "== samba myshare =="
curl -fsS "$BASE/system/storage/samba/myshare" | python3 -m json.tool

echo "== samba status =="
curl -fsS "$BASE/system/storage/samba/status"; echo

echo "== service config plans =="
for id in smbd ftp sftp webdav; do
  echo "-- $id --"
  curl -fsS "$BASE/system/network/server/configPlan?id=$id" | python3 -m json.tool | head -n 80 || true
done
