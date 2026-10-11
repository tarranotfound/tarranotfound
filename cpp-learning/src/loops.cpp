#include <iostream>

int main() {
    int total = 0;
    for (int i = 1; i <= 10; ++i) total += i;
    std::cout << "Sum from 1 to 10: " << total << '\n';

    int countdown = 3;
    while (countdown > 0) std::cout << countdown-- << "...\n";
    std::cout << "Go!\n";
}
