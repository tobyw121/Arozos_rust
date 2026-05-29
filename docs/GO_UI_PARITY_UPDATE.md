# Go UI Parity Update

This build keeps the Rust backend/runtime from the previous working version and focuses on matching the original Go ArozOS WebApp presentation layer.

## What changed

- The bundled application HTML/CSS/JS/image assets remain the original ArozOS WebApp assets under `resources/web`.
- Built-in module metadata in `src/app_api.rs` was aligned with the original `init.agi` registrations for visual fields such as `Name`, `Desc`, `Group`, `IconPath`, `Version`, `StartDir`, window sizes, embedded support and supported extensions.
- Module-local Go registration paths such as `index.html` and `img/module_icon.png` are normalized to web-root paths such as `Photo/index.html` and `Photo/img/module_icon.png`, so the Rust build stays standalone while rendering the original UI correctly.
- Embedded-only modules such as PDF Viewer are now handled by the Rust desktop launcher using their embedded window mode instead of forcing a non-Go full-window mode.
- The previous Rust no-tab desktop launcher is retained, but it now respects `SupportFW`/`SupportEmb` like the original module model.

## Asset coverage

The file `migration/go_ui_asset_manifest.json` contains SHA-256 hashes and sizes for the bundled app assets. This can be used to verify that the Rust build is serving a stable, self-contained UI asset tree.

## Runtime model

ArozOS apps are browser-based HTML/JS/CSS applications. In the original Go build, their server-side functions are provided by Go/AGI. In this Rust build:

- UI assets are served from `resources/web`.
- App backends are handled by native Rust routes in `src/app_api.rs` and compatibility handlers in `src/legacy_api.rs`.
- No Go source, Go binary or Go toolchain is needed at runtime.

