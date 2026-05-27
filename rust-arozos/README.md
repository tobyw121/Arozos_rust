# Rust ArOZ (AlpNAS Desktop)

Eine vollständige Portierung des [arozOS](https://github.com/tobychui/arozos) Projekts von Go nach Rust.

## Projektstruktur

Die Modulstruktur entspricht dem originalen Go-Projekt:

```
rust-arozos/
├── Cargo.toml              # Rust Paket-Konfiguration
├── src/
│   ├── main.rs             # Haupt-Einstiegspunkt (entspricht main.go)
│   ├── lib.rs              # Bibliotheks-Root
│   ├── error.rs            # Fehlerbehandlung (entspricht error.go)
│   ├── network.rs          # Netzwerk-Services (entspricht network.go)
│   ├── permission.rs       # Berechtigungen (entspricht permission.go)
│   ├── system.rs           # System-Informationen (entspricht system.go)
│   ├── mod/                # Module (entspricht src/mod/)
│   │   ├── agi/            # AGI Interface
│   │   ├── auth/           # Authentifizierung
│   │   ├── database/       # Datenbank-Abstraktion
│   │   ├── filesystem/     # Dateisystem-Abstraktionen
│   │   ├── network/        # Netzwerk-Dienste
│   │   ├── storage/        # Storage-Management
│   │   ├── user/           # Benutzerverwaltung
│   │   └── ...             # Weitere Module
│   └── system/             # System-Konfigurationen
├── config/                 # Konfigurationsdateien
├── docs/                   # Dokumentation
└── web/                    # Web-Frontend (wird von Go übernommen)
```

## Kernkomponenten

### Bereits portierte Module:
1. **Module Handler** (`src/mod/modules/`) - Modulverwaltung und Registrierung
2. **User Handler** (`src/mod/user/`) - Benutzerverwaltung
3. **Database** (`src/mod/database/`) - Key-Value Datenbank (Sled statt BoltDB)
4. **Filesystem** (`src/mod/filesystem/`) - Dateisystem-Abstraktionen

### Geplante Portierungen:
- HTTP Router & Middleware
- Authentication Gateway
- FTP/WebDAV Server
- Network Services (mDNS, UPnP, SSDP)
- Storage Bridge
- AGI Interface

## Build & Entwicklung

### Voraussetzungen
- Rust 1.75+ (`rustup install stable`)
- cargo

### Bauen
```bash
cargo build --release
```

### Entwickeln
```bash
cargo watch -x run
```

### Tests
```bash
cargo test
```

## Unterschiede zu Go

| Go | Rust |
|----|------|
| goroutines | tokio async/await |
| channels | tokio mpsc channels |
| interfaces | traits |
| structs with methods | structs + impl blocks |
| error handling (err != nil) | Result<T, E> + ? operator |
| defer | Drop trait / RAII |
| go modules | cargo crates |
| net/http | axum + hyper |
| BoltDB | sled |

## Lizenz

MIT License (wie das Originalprojekt)

## Mitwirken

Beiträge sind willkommen! Bitte orientiere dich an der bestehenden Modulstruktur.
