#include <iostream>
#include <sstream>
#include <string>

double calculate(double a, char op, double b) {
    switch (op) {
        case '+': return a + b;
        case '-': return a - b;
        case '*': return a * b;
        case '/':
            if (b == 0) throw std::runtime_error("division by zero");
            return a / b;
        default: throw std::runtime_error("unsupported operator");
    }
}

int main() {
    std::cout << "C++ Terminal Calculator\n";
    std::cout << "Enter an expression, for example: 8 + 3\n";

    std::string line;
    std::getline(std::cin, line);
    std::istringstream input(line);

    double a{}, b{};
    char op{};
    if (!(input >> a >> op >> b)) {
        std::cerr << "Invalid expression. Try: 8 + 3\n";
        return 1;
    }

    try {
        std::cout << "Result: " << calculate(a, op, b) << '\n';
    } catch (const std::exception& error) {
        std::cerr << "Error: " << error.what() << '\n';
        return 1;
    }
    return 0;
}
