# Learning goals

Goal: Rust for CLI tools, backend services and Flutter integration.
Based on the chapters of the Rust Book (doc.rust-lang.org/book), plus hands-on topics.

## Phase 1: The language
- [x] 1. Ownership, borrowing, `String` vs `&str`
- [x] 2. Structs: `impl`, methods, `Self`, `::new()` constructors
- [ ] 3. Enums with data, `match`, `if let`, `let else`, `Option`
- [ ] 4. Modules & files: `mod`, `pub`, `use`, `crate::`/`super::`, `foo.rs` vs `foo/mod.rs`, `lib.rs` + `main.rs`
- [ ] 5. Cargo & crates: dependencies (`cargo add`), crates.io, `Cargo.lock`, features, workspaces
- [ ] 6. Error handling: `panic!` vs `Result`, `?`, custom error types, `thiserror`/`anyhow`
- [ ] 7. Collections: `Vec`, `HashMap`, `HashSet`, strings & UTF-8
- [ ] 8. Closures & iterators: `map`/`filter`/`fold`, `Fn`/`FnMut`/`FnOnce`, `move`
- [ ] 9. Traits: custom traits, `derive`, standard traits (`Display`, `From`/`Into`, `Default`, `Iterator`)
- [ ] 10. Generics, trait bounds, `impl Trait` vs `dyn Trait`
- [ ] 11. Lifetimes
- [ ] 12. Testing & docs: unit tests, integration tests (`tests/`), doc tests, `cargo doc`
- [ ] 13. Smart pointers: `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`
- [ ] 14. Concurrency: threads, channels, `Send`/`Sync`
- [ ] 15. Using macros and writing simple `macro_rules!`
- [ ] 16. `unsafe` & FFI basics (groundwork for Flutter)

## Phase 2: Build a CLI tool
- [ ] Arguments with `clap`, reading/writing files, environment variables
- [ ] Config & JSON with `serde`
- [ ] Project: a small CLI tool of my own, cleanly split into modules

## Phase 3: Backend
- [ ] `async`/`await`, futures, `tokio`
- [ ] HTTP API with `axum`
- [ ] Database with `sqlx`
- [ ] Logging with `tracing`

## Phase 4: Flutter
- [ ] `flutter_rust_bridge`: setup, types across the language boundary
- [ ] Async & streams between Rust and Dart
- [ ] Builds for Android/iOS (cross-compiling)
