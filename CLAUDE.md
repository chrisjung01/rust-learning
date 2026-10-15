# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Projekt

Rust-Lernprojekt. Frisches `cargo new`-Binary (Edition 2024, keine Dependencies), bisher nur `src/main.rs` mit Hello World.
Lernziele gehören in `LERNZIELE.md` (aktuell leer).

## Befehle

```sh
cargo run                 # bauen + ausführen
cargo build
cargo test                # alle Tests
cargo test <name>         # einzelner Test (Substring-Match auf Testnamen)
cargo clippy              # Lints
cargo fmt                 # formatieren
```
