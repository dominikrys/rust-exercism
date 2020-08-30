# Exercism Rust Exercises

My solutions and notes on problems in the Exercism Rust track.

## Useful commands

Run first test

```bash
$ cargo test
```

Run all ignored tests

```bash
$ cargo test -- --ignored
```

Run a specific test

```bash
$ $ cargo test some_test
```

To see, if your solution contains some common ineffective use cases, inside the solution directory use

```bash
$ cargo clippy --all-targets
```