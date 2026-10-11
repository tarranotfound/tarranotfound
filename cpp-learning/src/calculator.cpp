#include <iostream>

int main() {
    double a{}, b{};
    char op{};
    std::cout << "Expression (e.g. 8 * 3): ";
    if (!(std::cin >> a >> op >> b)) {
        std::cerr << "Invalid input\n";
        return 1;
    }
    switch (op) {
        case '+': std::cout << a + b << '\n'; break;
        case '-': std::cout << a - b << '\n'; break;
        case '*': std::cout << a * b << '\n'; break;
        case '/':
            if (b == 0) { std::cerr << "Cannot divide by zero\n"; return 1; }
            std::cout << a / b << '\n'; break;
        default: std::cerr << "Unsupported operator\n"; return 1;
    }
}
