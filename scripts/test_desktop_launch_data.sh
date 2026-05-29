#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
echo "Testing module list"
curl -fsS "$BASE/system/modules/list" | python3 -m json.tool >/tmp/arozos_modules.json
python3 - <<'PY'
import json, sys
mods=json.load(open('/tmp/arozos_modules.json'))
for name in ['Photo','Music','Memo','NotepadA','Code Studio','Web Downloader']:
    m=next((x for x in mods if x.get('Name')==name), None)
    print(name, 'OK' if m else 'MISSING', m.get('StartDir') if m else '')
PY
for m in Photo Music Memo NotepadA "Code Studio" "Web Downloader"; do
  echo "Testing getLaunchPara: $m"
  curl -fsS --get --data-urlencode "module=$m" "$BASE/system/modules/getLaunchPara" | python3 -m json.tool | head -20
done
