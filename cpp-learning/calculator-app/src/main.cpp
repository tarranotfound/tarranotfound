#include <iostream>
#include <sstream>
#include <stdexcept>
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
    std::cout << "Enter expressions like 8 + 3. Type quit to exit.\n";

    std::string line;
    while (true) {
        std::cout << "> ";
        if (!std::getline(std::cin, line)) break;
        if (line == "quit" || line == "exit") break;
        if (line.empty()) continue;

        std::istringstream input(line);
        double a{}, b{};
        char op{};
        if (!(input >> a >> op >> b)) {
            std::cout << "Invalid expression. Example: 8 + 3\n";
            continue;
        }

        try {
            std::cout << "= " << calculate(a, op, b) << '\n';
        } catch (const std::exception& error) {
            std::cout << "Error: " << error.what() << '\n';
        }
    }

    std::cout << "Goodbye!\n";
    return 0;
}
