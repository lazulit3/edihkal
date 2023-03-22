# Contributing to edihkal

## Local Development Setup

These use [`cargo-binstall`](https://crates.io/crates/cargo-binstall) to install only binary release artifacts instead of compiling packages from source.
To install tools by building them from source code, use `cargo install` instead.

Install `cargo-binstall` (if installing binaries):
```sh
cargo install cargo-binstall
```

Install [`cargo-hack`](https://crates.io/crates/cargo-hack) for running tests:
```sh
cargo binstall cargo-hack
```

## Testing

### Start Database Container

Start a database container for local development:

```sh
./edihkal/localdev/init_db.sh
```

### Run Tests Using `cargo-hack`

edihkal uses [`cargo-hack`](https://crates.io/crates/cargo-hack) to run tests against each crate independently and to test different combinations of feature flags.
Running only `cargo test --workspace` is faster, but may not identify all failures.

Run tests with default feature flags:

```sh
cargo hack test
```

Run tests for each combinations of feature flags:

```sh
cargo hack test --each-feature
```
