# ArozOS Rust App/Auth Parity Fixes

This update improves parity with the Go ArozOS runtime in the areas that were causing visible runtime regressions:

- First-run user creation now displays a password policy hint on `user.html`.
- Server-side registration enforces the same policy and returns an explanatory error if it fails.
- `/system/auth/passwordPolicy` exposes the password policy to frontend code.
- System module icons and launch paths for System Setting and Users were corrected.
- More built-in SystemAO modules are registered in the module list so they appear in the Start Menu.
- Photo backend output was changed to the Go AGI shape: `[folders, filesWithSize]`.
- Music backend output was changed to the Go AGI shape for `listSong=all`: `{ cached, list }`.
- Music file info now returns `[filename, vpath, humanSize, byteSize, modifiedTime]`.
- Photo compressed-image handler now returns a plain data URL string like the AGI script.
- Web Downloader now returns JSON string `"OK"` while recording a `.url` file instead of executing an unattended network download.

Known gated operations remain intentionally protected: arbitrary AGI execution, external process execution, destructive disk/RAID actions, poweroff/reboot and unattended remote downloads.
