#!/usr/bin/env bash
set -euo pipefail
BASE=${1:-http://127.0.0.1:8080}
AUTH=${2:-}
CURL=(curl -fsS)
if [ -n "$AUTH" ]; then CURL+=( -u "$AUTH" ); fi

echo '== WebDAV status =='
"${CURL[@]}" "$BASE/system/network/webdav/status"; echo

echo '== WebDAV access config =='
"${CURL[@]}" "$BASE/system/network/webdav/accessConfig"; echo

echo '== WebDAV OPTIONS =='
curl -i -X OPTIONS "$BASE/webdav/" | sed -n '1,15p'

echo '== WebDAV browser index title =='
if [ -n "$AUTH" ]; then
  curl -fsS -u "$AUTH" "$BASE/webdav/" | grep -E 'ArozOS WebDAV|Access:' | head -5 || true
else
  CODE=$(curl -sS -o /tmp/arozos_webdav_index_test.html -w '%{http_code}' "$BASE/webdav/" || true)
  if [ "$CODE" = "401" ]; then
    echo '401 auth required: pass USER:PASS as second argument to test the protected browser index.'
  else
    grep -E 'ArozOS WebDAV|Access:' /tmp/arozos_webdav_index_test.html | head -5 || true
  fi
fi

echo '== File manager roots / host drives =='
"${CURL[@]}" "$BASE/system/file_system/listRoots"; echo
"${CURL[@]}" "$BASE/system/file_system/listMounts"; echo

echo '== Nginx reverse proxy config preview =='
"${CURL[@]}" "$BASE/system/network/nginx/configPreview?domain=arozos.local" | head -c 1200; echo
