#!/usr/bin/env bash
set -euo pipefail

# ArozOS Rust file-service helper.
# It installs/starts native OS daemons for services that cannot be implemented as a safe in-process Rust protocol:
# - Samba/SMB uses smbd/nmbd
# - SFTP uses OpenSSH sshd
# - FTP uses vsftpd
# WebDAV is built into arozos-rs at /webdav/ and does not require an extra daemon.

if [[ ${EUID:-$(id -u)} -ne 0 ]]; then
  echo "Run as root: sudo $0" >&2
  exit 1
fi

if command -v apt-get >/dev/null 2>&1; then
  apt-get update
  apt-get install -y samba openssh-server vsftpd
  systemctl enable --now smbd || true
  systemctl enable --now nmbd || true
  systemctl enable --now ssh || systemctl enable --now sshd || true
  systemctl enable --now vsftpd || true
elif command -v apk >/dev/null 2>&1; then
  apk add --no-cache samba samba-common-tools openssh vsftpd
  rc-update add samba default || true
  rc-update add sshd default || true
  rc-update add vsftpd default || true
  rc-service samba start || true
  rc-service sshd start || true
  rc-service vsftpd start || true
elif command -v pacman >/dev/null 2>&1; then
  pacman -Sy --needed --noconfirm samba openssh vsftpd
  systemctl enable --now smb || systemctl enable --now smbd || true
  systemctl enable --now nmb || systemctl enable --now nmbd || true
  systemctl enable --now sshd || true
  systemctl enable --now vsftpd || true
else
  echo "Unsupported package manager. Install samba, openssh-server and vsftpd manually." >&2
  exit 2
fi

echo "Done. Start ArozOS Rust with: sudo AROZOS_RS_ENABLE_SYSTEM_SERVICE_OPS=1 cargo run -- --host 0.0.0.0 --port 8080"
