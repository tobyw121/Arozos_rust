# Deep Port Update

This package continues the Go-to-Rust migration from the previous `arozos-rust-rest-ported` build.

## Newly implemented Rust handlers

The legacy HTTP compatibility layer no longer returns `StatusCode::NOT_IMPLEMENTED` for the previously listed open REST endpoints. The following areas received concrete Rust-side handlers:

- LDAP login/password endpoints with local fallback and JSON-backed configuration.
- OAuth login/authorize endpoint with provider config, local mock mode, and state generation.
- ZIP list/extract/create operations using a Rust ZIP backend.
- Module installation from ZIP into the Rust system module directory with an installation manifest.
- Wi-Fi profile state management for connect/remove/power operations.
- Subservice state management for start/kill/list without blindly spawning untrusted processes.
- Self-update state recording for download/restart requests.
- IoT device command/nickname/icon state recording.
- AGI execution gate with explicit `AROZOS_RS_ENABLE_AGI_EXEC=1` opt-in.
- Wake-on-LAN magic packet sender.
- Wallpaper asset lookup.
- Password reset validation/confirmation using JSON-backed reset keys.
- AlpNAS installer plan recording.
- Disk/RAID/Power operation command planning behind explicit privileged gates.

## Safety gates that intentionally remain

The following operations are represented as Rust command planners or state machines, but are not executed by default because they can destroy data, power off the host, or run arbitrary code:

- disk mount/unmount/format
- RAID create/remove/grow/assemble/format
- AGI script execution
- poweroff/reboot
- real OS Wi-Fi association
- external OAuth token exchange
- live LDAP network bind
- process supervisor spawning/killing

To execute the dangerous command planners, both the relevant runtime configuration and environment variables must be enabled. For example:

- `AROZOS_RS_ENABLE_DESTRUCTIVE_DISK_OPS=1`
- `AROZOS_RS_ENABLE_POWER_OPS=1`
- `AROZOS_RS_ENABLE_AGI_EXEC=1`

## Current status

All 255 discovered HTTP routes remain registered. The REST layer has been deepened from compatibility stubs to native Rust handlers for the remaining low-risk and medium-risk operations. The generated `src/ported/*.rs` files still preserve the one-file-per-Go-source migration map; many of those function-level stubs are retained as source-coverage markers, not as the active HTTP implementation.

A local `cargo build` could not be run in this sandbox because no Rust toolchain is installed.
