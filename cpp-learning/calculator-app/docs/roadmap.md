# Learning roadmap

This project is intentionally small and evolves in stages.

## Current baseline

- Read one expression per line.
- Support binary `+`, `-`, `*`, `/`, and `^` in the current three-token format.
- Support `sqrt` as a separate command.
- Keep a session history and provide `help`, `history`, `clear`, and `quit` commands.

## Next milestones

1. Separate parsing from calculation.
2. Add automated tests for invalid input and numeric edge cases.
3. Support operator precedence and parentheses with a real expression parser.
4. Separate the calculation engine from the terminal interface.
5. Prototype an optional keyboard-driven TUI after the engine is testable.

The roadmap describes future work; items beyond the current baseline are not implemented yet.
