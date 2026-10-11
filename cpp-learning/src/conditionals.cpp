#include <iostream>

int main() {
    int score{};
    std::cout << "Enter score (0-100): ";
    if (!(std::cin >> score) || score < 0 || score > 100) {
        std::cerr << "Score must be between 0 and 100\n";
        return 1;
    }
    if (score >= 90) std::cout << "Excellent\n";
    else if (score >= 75) std::cout << "Passed\n";
    else std::cout << "Keep practicing\n";
}
