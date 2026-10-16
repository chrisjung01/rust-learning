# rust-learning

I'm learning Rust, coming from Dart and Flutter. This repository holds the exercises: one session per topic, and every session is checked by tests.

## Why Rust, and the plan

I want Rust for three things: command-line tools, backend services, and native code inside Flutter apps. The path follows the chapters of the [Rust Book](https://doc.rust-lang.org/book/) first, then moves to real projects:

1. **The language.** 16 topics, from ownership to `unsafe` and FFI.
2. **A CLI tool.** `clap`, `serde`, and a small tool of my own split into modules.
3. **Backend.** `tokio`, an HTTP API with `axum`, `sqlx`, `tracing`.
4. **Flutter.** `flutter_rust_bridge`, async and streams between Rust and Dart, Android and iOS builds.

Every topic and its progress is in [LEARNING_GOALS.md](LEARNING_GOALS.md).

## How a session works

I work with [Claude Code](https://claude.com/claude-code) as a tutor. Each session goes through the same steps:

1. **Theory first.** I start `/cj-teach <topic>`, a Claude Code skill that explains the topic in depth: the reasoning behind it, examples, and how it compares to Dart. Only I can start it; Claude reminds me if I forget.
2. **Exercises.** Claude writes the exercises into `src/main.rs`: functions that still contain `todo!()`, plus tests that describe what they should do. `cargo test` is red.
3. **Tag the problem.** That state gets committed and tagged `session-NN-problem`.
4. **Solve.** I fill in the functions until `cargo test` is green.
5. **Tag the solution.** The solved state gets tagged `session-NN-solution`, and the topic is ticked off in `LEARNING_GOALS.md`.

The explanations in the chat are in German, my native language. Everything in the repository (code, comments, commits, tags) is in English.

A ruleset on GitHub protects the `session-*` tags: once pushed, nobody can move or delete them, so the problem and solution states stay as they were.

### Example: session 1, ownership and borrowing

Session 1 has three exercises in `src/main.rs`.

**Exercise 1: make it compile without `.clone()`.** `main` creates a `String`, passes it to `make_greeting`, and prints it again afterwards. With `make_greeting` taking an owned `String`, the value moves into the function and the second `println!` fails to compile. The rule: change only the signature. The fix borrows instead of taking ownership:

```rust
fn make_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}
```

In Dart this never comes up. Every object is passed by reference and the garbage collector frees it later. In Rust every value has exactly one owner, passing a `String` by value hands that ownership over, and `&` lends the value without giving it away.

**Exercise 2: change a string in place.** `append_bang(s: &mut String)` appends `!` through a mutable borrow, without creating a new string. The test calls it twice and expects `"Hi!!"`.

**Exercise 3: return a slice.** `first_word(text: &str) -> &str` returns the first word as a slice that points into the input, so nothing is copied. The tests cover `"hello world"`, a single word, and the empty string.

Session 1 only has `session-01-solution`. The two-tag convention started after it, so its problem state was never committed.

### Browsing the sessions

```sh
git tag -l 'session-*'                              # list all sessions
git checkout session-01-solution                    # look at a solved session
git diff session-02-problem session-02-solution     # from session 2 on: what solving changed
```

## How this repository is maintained

Changes to the repository itself, beyond the exercises, go through a spec-driven flow with my Claude Code skills from [chrisjung01/agent-skills](https://github.com/chrisjung01/agent-skills). Making this repository public was the first run:

1. **Decide.** `/cj-grill-with-docs` asked one decision at a time until I confirmed the list: public, MIT license, English content, no CI, pull requests only from collaborators.
2. **Spec.** `/cj-to-spec` turned the decisions into issue [#1](https://github.com/chrisjung01/rust-learning/issues/1) with 14 acceptance criteria, each one a check with a yes or no answer, such as "`cargo publish --dry-run` prints `cannot be published`".
3. **Build.** `/cj-implement` made the changes on a branch, ran the checks, had a fresh reviewer read the diff, and opened pull request [#2](https://github.com/chrisjung01/rust-learning/pull/2).
4. **Ask before acting.** Every step other people can see, like a push, a tag, the switch to public or a ruleset, waited for my yes.

There is no CI on purpose: every `session-NN-problem` commit has red tests by design, so a CI run would flag every new session as broken.

## Contributing

Only I push to this repository, and only collaborators can open pull requests. Issues are welcome: a question, an idea for a better exercise, or a mistake in an explanation.

## Commands

```sh
cargo run      # run
cargo test     # check the exercises
cargo clippy   # lints
```

## License

[MIT](LICENSE)
