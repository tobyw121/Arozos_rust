#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"

echo "== Platform adapter =="
curl -fsS "$BASE/system/network/server/platform" | python3 -m json.tool | head -80 || true

echo "== Service list =="
curl -fsS "$BASE/system/network/server/list" | python3 -m json.tool | head -120 || true

for svc in samba ftp sftp webdav tftp; do
  echo "== Native status: $svc =="
  curl -fsS "$BASE/system/network/server/osstatus?id=$svc" | python3 -m json.tool | head -80 || true
done

echo "== Endpoints =="
curl -fsS "$BASE/system/network/server/endpoints" | python3 -m json.tool | head -120 || true
