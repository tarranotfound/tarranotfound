# Testing

Run the smoke test from the calculator project directory:

```sh
make test
```

Build and launch the application with:

```sh
make
./build/calculator
```

The smoke test compiles `src/main.cpp` into a temporary directory, feeds a small set of expressions to the program, and checks for expected results. It does not require the generated binary to be committed.

When changing behavior, update the smoke test and manually check invalid input, division by zero, negative square roots, and history commands.
