# C++ Learning

A collection of small C++17 exercises, from language basics to a terminal todo manager.

## Requirements

- A C++17 compiler such as `g++`
- GNU Make (optional)

## Build every example

```sh
make
```

Binaries are created in `build/`. Run one, for example:

```sh
./build/hello
./build/calculator
./build/todo
```

Build a single example manually:

```sh
g++ -std=c++17 -Wall -Wextra -pedantic src/hello.cpp -o hello
./hello
```

The examples cover variables, conditions, loops, functions, vectors, strings, file I/O, and a small interactive todo manager.
