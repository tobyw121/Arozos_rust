# System Settings and Service UI Fix

This build fixes the ArozOS System Setting window and service configuration pages.

Changes:

- `/system/setting/list` now uses the Rust legacy/API compatibility layer instead of the generic config endpoint.
- The endpoint now returns Go-compatible setting groups and setting modules.
- Start menu entries for Samba, FTP, WebDAV, Disk Space, Network, Wi-Fi, Logs and Users now launch the original System Setting shell with the correct tab hash, instead of opening fragment pages without Semantic UI context.
- Network file server endpoints now return Go-compatible arrays/maps for `SystemAO/disk/services.html`.
- Samba endpoints now return Go-compatible arrays and field names (`Name`, `Path`, `ValidUsers`, `UnixUsername`, etc.).
- FTP status now returns the fields expected by `SystemAO/disk/ftp.html`.
- WebDAV status/list endpoints now return the array/list shapes expected by `SystemAO/disk/webdav.html`.
- Disk Space and Space Finder now receive Go-compatible arrays.
- Network hardware info now returns `Name`, `HardwareAddr`, `IPv4Addr`, and `IPv6Addr`.

Hardware/destructive operations remain gated; UI controls persist Rust-side state where safe.
