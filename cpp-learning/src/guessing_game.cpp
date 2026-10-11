#include <iostream>

int main() {
    constexpr int secret = 7;
    int guess{};
    std::cout << "Guess the number (1-10): ";
    while (std::cin >> guess) {
        if (guess == secret) { std::cout << "Correct!\n"; return 0; }
        std::cout << (guess < secret ? "Too low. Try again: " : "Too high. Try again: ");
    }
    std::cerr << "Input ended\n";
    return 1;
}
