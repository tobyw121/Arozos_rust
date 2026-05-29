# WebDAV compile fix v0.13.2

Fixed Rust string escaping in `src/webdav_server.rs` for the styled WebDAV directory index.

Changes:
- HTML fragments now use raw string literals.
- `Content-Disposition` filename quoting is escaped correctly.
- Windows path separator replacement uses `replace('\\', "/")`.
- Version bumped to `0.13.2`.
