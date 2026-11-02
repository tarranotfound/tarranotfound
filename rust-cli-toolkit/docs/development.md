# Development notes

## Common commands

```sh
# Run the application
cargo run

# Run automated tests
cargo test

# Check compilation without producing a runnable binary
cargo check

# Apply standard Rust formatting
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

## Contribution guidelines

- Keep each utility's core logic in its own module.
- Prefer small functions that can be tested without interactive terminal input.
- Return `Result` when an operation can fail for a meaningful reason.
- Keep user-facing prompts and error messages concise.
- Run formatting, compilation checks, and tests before submitting changes.

The project currently has no external crate dependencies.