#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
need(){ command -v "$1" >/dev/null 2>&1 || { echo "missing command: $1" >&2; exit 1; }; }
need curl
need python3

echo "Testing module metadata and original app asset URLs"
json="$(curl -fsS "$BASE/system/modules/list")"
python3 - "$BASE" "$json" <<'PY'
import json, sys, subprocess, urllib.parse
base=sys.argv[1].rstrip('/')
mods=json.loads(sys.argv[2])
required=["Browser","Camera","Clock","Code Studio","MDEditor","Management Gateway","Manga","Memo","Music","NotepadA","OfficeViewer","OnScreenKeyboard","PDF Viewer","Paint","Photo","Recorder","Serverless","Speedtest","Timer","Unit Tester","Video","Web Builder","Web Downloader"]
by={m.get('Name'):m for m in mods}
missing=[n for n in required if n not in by]
if missing:
    raise SystemExit('missing modules: '+', '.join(missing))

def url(path):
    if not path:
        return None
    return base + '/' + urllib.parse.quote(path, safe='/:?#&=%')

for name in required:
    m=by[name]
    for field in ('IconPath','LaunchFWDir','StartDir','LaunchEmb'):
        p=m.get(field) or ''
        if not p or p == 'about:blank':
            continue
        u=url(p)
        code=subprocess.check_output(['curl','-o','/dev/null','-sS','-w','%{http_code}',u], text=True).strip()
        if code not in ('200','206','304'):
            raise SystemExit(f'{name} {field} not reachable: {p} -> HTTP {code}')
    print(f'{name}: OK')
print('Go UI parity smoke test OK')
PY
