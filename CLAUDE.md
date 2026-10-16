# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Rust learning project (edition 2024, no dependencies). Each session's exercises live in `src/main.rs`, verified by tests.
Topics and progress are tracked in `LEARNING_GOALS.md`.

## Language

- Code, comments, docs, commit messages and tags are in English.
- Explanations in the chat are in German.

## Sessions

- **Theory first:** Every new session starts with `/cj-teach <topic from LEARNING_GOALS.md>` to cover the theory in depth. Exercises in `src/main.rs` come only after that. Only the user can start the skill (`disable-model-invocation`); Claude reminds the user at the start of a session if it has not run yet.
- **Thorough explanations:** Explain concepts in depth, with the reasoning behind them, examples and comparisons to Dart. Too much is better than too little.
- **Two git tags per session:**
  - `session-NN-problem`: the state with the exercises (code with `todo!()`, tests red), before solving
  - `session-NN-solution`: the state after solving (tests green)
  - `NN` has two digits (`session-02-problem`). Session 1 only has `session-01-solution`; its problem state was never committed.
  - A GitHub ruleset blocks moving or deleting `session-*` tags, so tag the right commit before pushing.

## Commands

```sh
cargo run                 # build + run
cargo build
cargo test                # all tests
cargo test <name>         # single test (substring match on test names)
cargo clippy              # lints
cargo fmt                 # format
```
