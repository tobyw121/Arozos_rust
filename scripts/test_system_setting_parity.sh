#!/usr/bin/env bash
set -euo pipefail
BASE=${1:-http://127.0.0.1:8080}
echo "Testing /system/setting/list root groups"
curl -fsS "$BASE/system/setting/list" | python3 -m json.tool | head -40
echo "Testing /system/setting/list?listGroup=Network"
curl -fsS "$BASE/system/setting/list?listGroup=Network" | python3 -m json.tool | head -80
echo "Testing file server services"
curl -fsS "$BASE/system/network/server/list" | python3 -m json.tool | head -80
echo "Testing Samba responses"
curl -fsS "$BASE/system/storage/samba/listUsers" | python3 -m json.tool
curl -fsS "$BASE/system/storage/samba/list" | python3 -m json.tool
echo "Testing Disk Space"
curl -fsS "$BASE/system/disk/space/list" | python3 -m json.tool | head -60
