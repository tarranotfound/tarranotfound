# Build on Arch Linux

Install the basic toolchain:

```sh
sudo pacman -S --needed base-devel git
```

Clone the repository and enter the calculator directory:

```sh
git clone https://github.com/tarranotfound/tarranotfound.git
cd tarranotfound/cpp-learning/calculator-app
```

Build, test, and run:

```sh
make
make test
./build/calculator
```

The current baseline uses the C++ standard library and does not need a third-party UI dependency. A future full-screen TUI may add one.
