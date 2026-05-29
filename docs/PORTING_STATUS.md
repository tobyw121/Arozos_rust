# Porting Status

This repository is a project-wide Rust rewrite scaffold generated from the supplied ArozOS/AlpNAS Go archive.

## Covered automatically

- Every Go source file has a generated Rust migration module under `src/ported/`.
- Every detected Go function/method has a typed Rust async stub returning `LegacyPortError::NotYetPorted`.
- Every detected Go type/interface declaration is recorded in module metadata.
- 255 unique HTTP endpoints are registered in the Axum router.
- Runtime `web/` and `system/` assets are copied to `resources/` so the server can serve the existing frontend while backend modules are being replaced.
- The main startup path, CLI flags, auth skeleton, system info endpoint, static file serving, graceful shutdown and endpoint registry are native Rust.

## Not yet equivalent to upstream Go behavior

The original codebase contains OS storage management, WebDAV/TFTP/FTP, disk/RAID commands, AGI runtime integration, user permission logic, share links, media transcoding, LDAP/OAuth, IoT discovery and many other side-effect-heavy services. These require manual Rust implementation and platform testing. The generated stubs intentionally return HTTP 501 so that missing behavior is explicit and safe.

## Recommended manual port order

1. `mod/database`, `mod/auth`, `mod/user`, `mod/permission`
2. `mod/filesystem`, `storage.go`, `file_system.go`, share handling
3. Static web routing and module management
4. Disk/RAID/hardware/network services behind feature flags
5. AGI/serverless runtime and subservices
6. Media, WebDAV/FTP/TFTP and websocket features

## Validation

No Rust toolchain was available in this execution environment, so `cargo build` was not run here. The project is written as standard Rust 2021 with a normal Cargo manifest.


## Native Rust modules added in this package

This second pass adds concrete Rust handlers for the migration-critical core:

- authentication session skeleton: `/system/auth/login`, `/api/auth/login`, `/system/auth/logout`, `/system/auth/checkLogin`
- system identity/runtime endpoints: `/system/id/ping`, `/system/id/requestInfo`, `/system/info/getArOZInfo`, `/system/info/getCPUinfo`, `/system/info/getRAMinfo`, `/system/info/getRuntimeInfo`, `/system/info/getDriveStat`, `/system/info/ifconfig`, `/system/info/license`
- boot/settings views: `/system/bootflags`, `/system/setting/list`
- local filesystem base operations: `/system/file_system/listRoots`, `/system/file_system/listDrives`, `/system/file_system/listDir`, `/system/file_system/getProperties`, `/system/file_system/newItem`, `/system/file_system/fileOpr`, `/system/file_system/search`

The original Go source tree is also copied under `migration/original_go_source/` as a porting reference. It is not used at runtime.
