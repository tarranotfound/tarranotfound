#include <cmath>
#include <iomanip>
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
        case '^': return std::pow(a, b);
        default: throw std::runtime_error("unsupported operator");
    }
}

void print_help() {
    std::cout << "Commands: help, quit\n"
              << "Expressions: 8 + 3, 12 / 4, 2 ^ 8\n";
}

int main() {
    std::cout << std::fixed << std::setprecision(4);
    std::cout << "C++ Terminal Calculator\nType help for usage.\n";
    std::string line;

    while (true) {
        std::cout << "> ";
        if (!std::getline(std::cin, line)) break;
        if (line == "quit" || line == "exit") break;
        if (line == "help") { print_help(); continue; }
        if (line.empty()) continue;

        std::istringstream input(line);
        double a{}, b{};
        char op{};
        if (!(input >> a >> op >> b)) {
            std::cout << "Invalid expression. Type help for examples.\n";
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
