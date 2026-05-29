#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
USERPASS="${2:-}"

echo "Testing canonical WebDAV OPTIONS"
curl -i -X OPTIONS "$BASE/webdav/" | sed -n '1,20p'

echo
echo "Testing compatibility alias WebDAV OPTIONS"
curl -i -X OPTIONS "$BASE/webdave/" | sed -n '1,20p'

if [[ -n "$USERPASS" ]]; then
  echo
  echo "Testing canonical PROPFIND"
  curl -i -X PROPFIND -H 'Depth: 1' -u "$USERPASS" "$BASE/webdav/" | sed -n '1,40p'

  echo
  echo "Testing alias PROPFIND"
  curl -i -X PROPFIND -H 'Depth: 1' -u "$USERPASS" "$BASE/webdave/" | sed -n '1,40p'
else
  echo
  echo "Pass USER:PASS as second argument to test authenticated PROPFIND."
  echo "Example: $0 $BASE admin:password"
fi
