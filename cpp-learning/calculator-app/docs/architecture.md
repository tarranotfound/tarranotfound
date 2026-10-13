# Code structure

The baseline application keeps its implementation in `src/main.cpp`.

- `calculate(a, op, b)` dispatches supported binary operators.
- `square_root(value)` validates the domain before calling `std::sqrt`.
- `print_help()` documents commands and examples.
- `main()` owns the input loop, command dispatch, result formatting, and in-memory history.

## Suggested next refactor

Move calculation and parsing into a small library independent of `std::cin` and `std::cout`. That will make it easier to test expressions without scripting terminal input, and it will let a future TUI reuse the same engine.
