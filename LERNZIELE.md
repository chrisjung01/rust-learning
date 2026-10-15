# Lernziele

Ziel: Rust für CLI-Tools, Backend und Flutter-Integration.
Grundlage: Kapitel des Rust Books (doc.rust-lang.org/book), ergänzt um Praxis-Themen.

## Phase 1: Sprache
- [x] 1. Ownership, Borrowing, `String` vs `&str`
- [ ] 2. Structs: `impl`, Methoden, `Self`, `::new()`-Konstruktoren
- [ ] 3. Enums mit Daten, `match`, `if let`, `let else`, `Option`
- [ ] 4. Module & Dateien: `mod`, `pub`, `use`, `crate::`/`super::`, `foo.rs` vs `foo/mod.rs`, `lib.rs` + `main.rs`
- [ ] 5. Cargo & Crates: Dependencies (`cargo add`), crates.io, `Cargo.lock`, Features, Workspaces
- [ ] 6. Fehlerbehandlung: `panic!` vs `Result`, `?`, eigene Error-Typen, `thiserror`/`anyhow`
- [ ] 7. Collections: `Vec`, `HashMap`, `HashSet`, Strings & UTF-8
- [ ] 8. Closures & Iteratoren: `map`/`filter`/`fold`, `Fn`/`FnMut`/`FnOnce`, `move`
- [ ] 9. Traits: eigene Traits, `derive`, Standard-Traits (`Display`, `From`/`Into`, `Default`, `Iterator`)
- [ ] 10. Generics, Trait Bounds, `impl Trait` vs `dyn Trait`
- [ ] 11. Lifetimes
- [ ] 12. Testing & Doku: Unit-, Integrations- (`tests/`), Doc-Tests, `cargo doc`
- [ ] 13. Smart Pointer: `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`
- [ ] 14. Concurrency: Threads, Channels, `Send`/`Sync`
- [ ] 15. Makros benutzen und einfache `macro_rules!` schreiben
- [ ] 16. `unsafe` & FFI-Grundlagen (Vorbereitung für Flutter)

## Phase 2: CLI-Tool bauen
- [ ] Argumente mit `clap`, Dateien lesen/schreiben, Umgebungsvariablen
- [ ] Config & JSON mit `serde`
- [ ] Projekt: eigenes kleines CLI-Tool, sauber in Module aufgeteilt

## Phase 3: Backend
- [ ] `async`/`await`, Futures, `tokio`
- [ ] HTTP-API mit `axum`
- [ ] Datenbank mit `sqlx`
- [ ] Logging mit `tracing`

## Phase 4: Flutter
- [ ] `flutter_rust_bridge`: Setup, Typen über die Sprachgrenze
- [ ] Async & Streams zwischen Rust und Dart
- [ ] Builds für Android/iOS (Cross-Compiling)
