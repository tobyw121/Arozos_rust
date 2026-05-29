# v0.13.3 WebDAV permissions, Nginx HTTPS proxy, File Manager drives

## WebDAV

The native Rust WebDAV endpoint remains canonical at `/webdav/`. It now has a persistent access configuration stored at:

`resources/system/rust-port-data/webdav_access.json`

Supported configuration fields:

- `allowed_paths`: virtual path prefixes that can be exposed, for example `user:/`.
- `denied_paths`: virtual path prefixes that are always blocked.
- `rules`: user/group rules with `read` and `write` booleans.
- `allow_admin_all`: administrator users can access all WebDAV paths.
- `allow_browser_index`: enables the styled browser directory index.
- `require_auth`: requires ArozOS auth / Basic auth.

Endpoints:

- `GET /system/network/webdav/accessConfig`
- `POST /system/network/webdav/accessConfig` with a full JSON config
- `/system/network/webdav/addPath?path=user:/Photo`
- `/system/network/webdav/removePath?path=user:/Photo`
- `/system/network/webdav/addRule?path=user:/Photo&group=user&read=true&write=false`
- `/system/network/webdav/removeRule?path=user:/Photo`

## Nginx HTTPS reverse proxy

Nginx is handled as a reverse proxy configuration layer, not as a Rust replacement for TLS. Generate a config with:

`/system/network/nginx/configPreview?domain=example.local&upstream=127.0.0.1:8080`

or write a config template into the Rust data directory:

`/system/network/nginx/writeConfig?domain=example.local`

Install system-wide with:

```bash
sudo ./scripts/setup_nginx_https_reverse_proxy.sh example.local 127.0.0.1:8080 /path/fullchain.pem /path/privkey.pem
```

## File Manager / Thunar-like drives

`/system/file_system/listRoots` and `/system/file_system/listMounts` now expose:

- ArozOS user root: `user:/`
- Linux mount points from `/proc/self/mountinfo`
- macOS volumes from `/Volumes`
- Windows drive letters

Host filesystem paths use the virtual prefix `host:/`. Reading is enabled by default. Writing to OS drives is disabled by default and requires:

```bash
AROZOS_RS_ENABLE_HOST_FS_WRITE=1 ./target/release/arozos-rs --host 0.0.0.0 --port 8080
```

This protects the host OS while still making mounted drives visible in the File Manager.
