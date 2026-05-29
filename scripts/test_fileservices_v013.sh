#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
AUTH="${2:-}"
echo "== Canonical WebDAV only =="
curl -i -X OPTIONS "$BASE/webdav/" | head -20
if curl -fsS "$BASE/webdave" >/tmp/webdave.out 2>/dev/null; then
  echo "ERROR: /webdave unexpectedly resolved"
  cat /tmp/webdave.out
  exit 1
else
  echo "OK: /webdave is not a protocol alias"
fi

echo "== Runtime status =="
for s in webdav ftp tftp sftp samba; do
  echo "-- $s"
  curl -fsS "$BASE/system/network/server/runtimeStatus?id=$s"; echo
done

echo "== Enable built-in Rust FTP/TFTP runtime =="
curl -fsS "$BASE/system/network/server/applyRuntime?id=ftp&enable=true"; echo
curl -fsS "$BASE/system/network/server/applyRuntime?id=tftp&enable=true"; echo

echo "== Service endpoints =="
curl -fsS "$BASE/system/network/server/endpoints"; echo
