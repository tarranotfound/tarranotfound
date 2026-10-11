#include <iostream>
#include <vector>

int main() {
    const std::vector<int> values{4, 8, 15, 16, 23, 42};
    int total = 0;
    for (const int value : values) {
        std::cout << value << ' ';
        total += value;
    }
    std::cout << "\nCount: " << values.size() << "\nSum: " << total << '\n';
}
