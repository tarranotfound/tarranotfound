CXX ?= g++
CXXFLAGS ?= -std=c++17 -Wall -Wextra -Wpedantic -O2
TARGET := build/calculator
SOURCE := src/main.cpp

.PHONY: all run clean test

all: $(TARGET)

$(TARGET): $(SOURCE)
	@mkdir -p build
	$(CXX) $(CXXFLAGS) $(SOURCE) -o $(TARGET)

run: $(TARGET)
	./$(TARGET)

test:
	bash tests/smoke.sh

clean:
	rm -rf build
