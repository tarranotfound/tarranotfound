# Input and error handling

Terminal input is centralized in the `validation` module.

- `read_line` reads one line from standard input and preserves I/O errors.
- `read_line_prompt` prints a prompt, flushes standard output, and then reads input.
- `read_number` parses a trimmed line as `f64` and returns a readable error when parsing fails.

Interactive tools handle input errors locally and return to the main menu instead of terminating the entire application. The calculator also reports unsupported operators and prevents division by zero.