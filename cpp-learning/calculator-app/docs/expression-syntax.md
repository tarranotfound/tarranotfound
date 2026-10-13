# Expression syntax

The current parser accepts a simple three-token binary expression:

```text
8 + 3
12 / 4
2 ^ 8
```

The square-root command uses a separate form:

```text
sqrt 81
```

Commands are entered on their own line: `help`, `history`, `clear`, `quit`, or `exit`.

## Current limitations

- Binary expressions require spaces between the operands and operator.
- Only one binary operation is accepted per line.
- Parentheses and chained expressions such as `2 + 3 * 4` are not supported yet.
- This is a command-line calculator, not yet a full-screen TUI.
