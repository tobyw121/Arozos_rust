#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"

echo "== File server overview =="
curl -fsS "$BASE/system/network/server/list" | python3 -m json.tool | head -80

echo "== FTP status =="
curl -fsS "$BASE/system/storage/ftp/status" | python3 -m json.tool

echo "== SFTP port/upnp/users =="
curl -fsS "$BASE/system/storage/sftp/port"; echo
curl -fsS "$BASE/system/storage/sftp/upnp"; echo
curl -fsS "$BASE/system/storage/sftp/users"; echo

echo "== WebDAV status/list =="
curl -fsS "$BASE/system/network/webdav/status" | python3 -m json.tool
curl -fsS "$BASE/system/network/webdav/list" | python3 -m json.tool
curl -fsS "$BASE/system/network/webdav/list?target=loggedin" | python3 -m json.tool

echo "== Samba status/list/users =="
curl -fsS "$BASE/system/storage/samba/status"; echo
curl -fsS "$BASE/system/storage/samba/list" | python3 -m json.tool
curl -fsS "$BASE/system/storage/samba/listUsers" | python3 -m json.tool

echo "== Host info formats =="
curl -fsS "$BASE/system/info/getCPUinfo" | python3 -m json.tool
curl -fsS "$BASE/system/info/getRAMinfo"; echo
curl -fsS "$BASE/system/info/getDriveStat" | python3 -m json.tool | head -60
curl -fsS "$BASE/system/info/ifconfig" | python3 -m json.tool
