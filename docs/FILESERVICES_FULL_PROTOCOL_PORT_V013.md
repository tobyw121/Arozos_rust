# File Services Full Protocol Port v0.13.0

This build removes the `/webdave` compatibility alias and uses canonical protocol endpoints only.

## Implemented natively in Rust

### WebDAV

Canonical mount:

```text
/webdav/
```

Supported HTTP/WebDAV methods:

- `OPTIONS`
- `PROPFIND`
- `GET`
- `HEAD`
- `PUT`
- `DELETE`
- `MKCOL`
- `MOVE`
- `COPY`

Authentication uses the ArozOS Rust user database via HTTP Basic Auth.

### FTP

A built-in passive FTP server is now available. It uses the ArozOS Rust user database and the configured ArozOS file root.

Default port is `2121` so it can run without root privileges. You may set it to `21` when running with appropriate privileges.

Implemented FTP commands include:

- `USER`, `PASS`, `QUIT`
- `SYST`, `FEAT`, `OPTS`, `NOOP`, `TYPE`
- `PWD`, `CWD`, `CDUP`
- `PASV`, `EPSV`
- `LIST`, `NLST`
- `RETR`, `STOR`
- `DELE`, `MKD`, `RMD`
- `RNFR`, `RNTO`
- `SIZE`, `MDTM`

### TFTP

A built-in TFTP server is now available.

Default port is `6969` so it can run without root privileges. You may set it to `69` when running with appropriate privileges.

Supported operations:

- RRQ / read
- WRQ / write
- DATA / ACK
- root-limited path resolution

TFTP has no authentication by protocol design, so expose it only on trusted networks.

## OS-backed protocols

### SFTP

SFTP is the SSH subsystem, not an HTTP endpoint. This build does not fake SFTP using aliases. It uses the OS `sshd` adapter/config plan.

Default port is `22`.

### Samba / SMB

SMB/CIFS is provided by the OS Samba daemon. This build does not fake SMB using HTTP aliases. It writes ArozOS state/config plans and can control the OS daemon when service control is enabled.

## Runtime endpoints

```text
/system/network/server/runtimeStatus?id=ftp
/system/network/server/runtimeStatus?id=tftp
/system/network/server/runtimeStatus?id=webdav
/system/network/server/runtimeStatus?id=sftp
/system/network/server/runtimeStatus?id=samba
/system/network/server/applyRuntime?id=ftp&enable=true
/system/network/server/applyRuntime?id=tftp&enable=true
```

## Important

FTP and TFTP are now native Rust protocol listeners. WebDAV remains the native Rust HTTP/WebDAV endpoint.
SFTP and Samba remain true OS protocol integrations because implementing SSH/SFTP and SMB/CIFS correctly inside ArozOS itself would require full protocol stacks and OS account/mount integration.
