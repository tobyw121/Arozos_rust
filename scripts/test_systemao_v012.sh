#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
echo "Testing SystemAO static aliases"
curl -fsS "$BASE/system/close.svg" >/dev/null
curl -fsS "$BASE/system_icon/folder.png" >/dev/null
curl -fsS "$BASE/system_setting/index.html" >/dev/null
curl -fsS "$BASE/system_settings.json" >/dev/null

echo "Testing SystemAO native Rust endpoints"
curl -fsS "$BASE/system/time/getTime" | head -c 200; echo
curl -fsS "$BASE/system/disk/alpnas/list" | head -c 500; echo
curl -fsS "$BASE/system/disk/devices/list" | head -c 500; echo
curl -fsS "$BASE/system/disk/devices/model?devName=sda" | head -c 200; echo
curl -fsS "$BASE/system/disk/raid/list" | head -c 200; echo
curl -fsS "$BASE/system/disk/smart/getSMART" | head -c 500; echo
curl -fsS "$BASE/system/disk/quota/quotaDist" | head -c 500; echo

echo "OK"
