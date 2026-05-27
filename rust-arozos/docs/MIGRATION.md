# Migrationsanleitung: Go zu Rust

Diese Datei dokumentiert die Portierung des arozOS-Projekts von Go nach Rust.

## Projektstruktur

Die Rust-Struktur spiegelt die Go-Struktur wider:

```
Go (Original)          Rust (Port)
────────────           ────────────
src/main.go         →  src/main.rs
src/module.go       →  src/mod/modules/
src/network.go      →  src/network.rs
src/permission.go   →  src/permission.rs
src/system.go       →  src/system.rs
src/mod/*           →  src/mod/*/
```

## Abhängigkeiten Mapping

| Go Package | Rust Crate | Zweck |
|------------|-----------|-------|
| `net/http` | `axum` + `hyper` | Web Server |
| `github.com/boltdb/bolt` | `sled` | Embedded KV-Database |
| `encoding/json` | `serde_json` | JSON Serialisierung |
| `flag` | `clap` | CLI Argument Parsing |
| `sync` | `tokio::sync` | Synchronisation |
| `context` | `tokio::task` | Async Context |
| `io/ioutil` | `tokio::fs` | File I/O |
| `os/signal` | `tokio::signal` | Signal Handling |
| `crypto/bcrypt` | `bcrypt` | Passwort Hashing |
| `github.com/gorilla/websocket` | `tokio-tungstenite` | WebSocket |

## Code-Migration Beispiele

### 1. Structs zu Rust Structs

**Go:**
```go
type ModuleInfo struct {
    Name         string
    Desc         string
    Group        string
    IconPath     string
    Version      string
    StartDir     string
    SupportFW    bool
}
```

**Rust:**
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub desc: String,
    pub group: String,
    pub icon_path: String,
    pub version: String,
    pub start_dir: String,
    pub support_fw: bool,
}
```

### 2. Error Handling

**Go:**
```go
err := doSomething()
if err != nil {
    return err
}
```

**Rust:**
```rust
do_something()?;  // ? operator propagates error
// oder
match do_something() {
    Ok(result) => result,
    Err(e) => return Err(e),
}
```

### 3. Goroutines zu Tokio Async

**Go:**
```go
go func() {
    doWork()
}()
```

**Rust:**
```rust
tokio::spawn(async move {
    do_work().await;
});
```

### 4. Interfaces zu Traits

**Go:**
```go
type Handler interface {
    ServeHTTP(w ResponseWriter, r *Request)
}
```

**Rust:**
```rust
pub trait Handler {
    fn serve_http(&self, req: Request) -> Result<Response>;
}
```

## Wichtige Unterschiede

### Speicherverwaltung
- **Go**: Garbage Collector
- **Rust**: Ownership & Borrowing (kein GC)

### Nebenläufigkeit
- **Go**: Goroutines + Channels
- **Rust**: async/await + Tokio Runtime

### Generics
- **Go**: Seit 1.18 verfügbar, begrenzt
- **Rust**: Sehr mächtiges Typsystem mit Generics

### Pattern Matching
- **Go**: switch statements
- **Rust**: match expressions (viel mächtiger)

## Fortschrittsübersicht

### ✅ Fertig portierte Module:
- [x] Grundgerüst (Cargo.toml, lib.rs, main.rs)
- [x] Error Handling (error.rs)
- [x] Startup Initialisierung (startup.rs)
- [x] Router Konfiguration (router.rs)
- [x] Module Handler (mod/modules/)
- [x] User Handler (mod/user/)
- [x] Database Abstraction (mod/database/)

### 🔄 In Arbeit:
- [ ] Authentication Gateway (mod/auth/)
- [ ] Filesystem Abstractions (mod/filesystem/)
- [ ] Storage Management (mod/storage/)

### ⏳ Geplant:
- [ ] Network Services (mod/network/)
- [ ] FTP/WebDAV Server
- [ ] AGI Interface (mod/agi/)
- [ ] Media Transcoder (mod/media/)
- [ ] IoT Handler (mod/iot/)
- [ ] Console (mod/console/)

## Build Instructions

### Voraussetzungen installieren
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
```

### Projekt bauen
```bash
cd rust-arozos
cargo build --release
```

### Tests ausführen
```bash
cargo test
```

### Development Mode
```bash
cargo watch -x run
```

## Nächste Schritte

1. **Authentication**: Login/Logout, Session Management
2. **Filesystem**: File Operations, Permissions
3. **Storage**: Pool Management, Mount/Unmount
4. **Network**: HTTP/HTTPS, WebSocket, mDNS
5. **Modules**: Module Loading, Installation

## Tipps für Contributors

1. **Verstehe das Original**: Lies die Go-Quellen bevor du portierst
2. **Teste häufig**: Schreibe Tests für jede Komponente
3. **Dokumentiere**: Halte die README aktuell
4. **Folge dem Stil**: Konsistente Code-Struktur ist wichtig
5. **Async wo sinnvoll**: Nicht alles muss async sein

## Lizenz

MIT License - wie das Originalprojekt
