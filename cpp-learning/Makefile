CXX ?= g++
CXXFLAGS ?= -std=c++17 -Wall -Wextra -Wpedantic -O2
SOURCES := $(wildcard src/*.cpp)
TARGETS := $(patsubst src/%.cpp,build/%,$(SOURCES))

.PHONY: all clean
all: $(TARGETS)

build/%: src/%.cpp
	@mkdir -p build
	$(CXX) $(CXXFLAGS) $< -o $@

clean:
	rm -rf build
