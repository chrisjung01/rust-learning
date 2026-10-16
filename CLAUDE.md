# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Projekt

Rust-Lernprojekt (Edition 2024, keine Dependencies). Übungen pro Session in `src/main.rs`, abgesichert durch Tests.
Themen und Fortschritt stehen in `LERNZIELE.md`.

## Sessions

- **Erklärungen ausführlich:** Konzepte gründlich erklären, mit dem Warum dahinter, Beispielen und Vergleich zu Dart. Lieber zu viel als zu knapp.
- **Zwei Git-Tags pro Session:**
  - `session-NN-problem` – Stand mit den Aufgaben (Code mit `todo!()`, Tests rot), vor dem Lösen
  - `session-NN-solution` – Stand nach dem Lösen (Tests grün)
  - `NN` zweistellig (`session-02-problem`). Session 1 hat nur `session-01-solution`, der Aufgaben-Stand wurde nie committet.

## Befehle

```sh
cargo run                 # bauen + ausführen
cargo build
cargo test                # alle Tests
cargo test <name>         # einzelner Test (Substring-Match auf Testnamen)
cargo clippy              # Lints
cargo fmt                 # formatieren
```
