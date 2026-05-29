#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
check() {
  local url="$1"
  local code
  code=$(curl -sS -o /tmp/arozos_icon_test.out -w '%{http_code}' "$BASE$url")
  if [[ "$code" != "200" ]]; then
    echo "FAIL $code $url"
    return 1
  fi
  if [[ ! -s /tmp/arozos_icon_test.out ]]; then
    echo "FAIL empty $url"
    return 1
  fi
  echo "OK $url"
}
check /img/public/vendor_icon.png
check /img/public/auth_bg.jpg
check /system/info/wallpaper.jpg
check /SystemAO/users/img/noprofileicon.png
check /SystemAO/users/img/users.svg
check /SystemAO/network/img/ethernet.svg
check /SystemAO/network/img/wifi.svg
check /SystemAO/system_setting/img/module.svg
check /SystemAO/system_setting/img/drive.svg
check /SystemAO/disk/raid/img/raid.svg
curl -sS "$BASE/system/info/getArOZInfo?icon=true" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert d.get("VendorIcon"), "missing VendorIcon"; assert d.get("HostName") is not None; print("OK getArOZInfo")'
curl -sS "$BASE/system/info/getRuntimeInfo" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert "StartupTime" in d and "ContinuesRuntime" in d; print("OK getRuntimeInfo")'
