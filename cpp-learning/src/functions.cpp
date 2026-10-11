#include <iostream>

int square(int value) { return value * value; }

int main() {
    for (int value = 1; value <= 5; ++value)
        std::cout << value << " squared is " << square(value) << '\n';
}
