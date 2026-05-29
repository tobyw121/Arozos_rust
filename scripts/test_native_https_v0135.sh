#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
HTTPS="${2:-https://127.0.0.1:8443}"

echo "== Native HTTPS config preview =="
curl -fsS "$BASE/system/network/https/configPreview?host=arozos.local" | python3 -m json.tool || true

echo "== Native HTTPS status over HTTP =="
curl -fsS "$BASE/system/network/https/status" | python3 -m json.tool || true

echo "== HTTPS root probe =="
curl -k -I "$HTTPS/" || true

echo "== HTTPS WebDAV OPTIONS probe =="
curl -k -i -X OPTIONS "$HTTPS/webdav/" || true
