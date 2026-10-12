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

double square_root(double value) {
    if (value < 0) throw std::runtime_error("square root of a negative number");
    return std::sqrt(value);
}

void print_help() {
    std::cout << "Commands: help, quit\n"
              << "Expressions: 8 + 3, 12 / 4, 2 ^ 8, sqrt 81\n";
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
        std::string first;
        input >> first;

        try {
            if (first == "sqrt") {
                double value{};
                if (!(input >> value)) throw std::runtime_error("usage: sqrt 81");
                std::cout << "= " << square_root(value) << '\n';
                continue;
            }

            std::istringstream expression(line);
            double a{}, b{};
            char op{};
            if (!(expression >> a >> op >> b)) {
                std::cout << "Invalid expression. Type help for examples.\n";
                continue;
            }
            std::cout << "= " << calculate(a, op, b) << '\n';
        } catch (const std::exception& error) {
            std::cout << "Error: " << error.what() << '\n';
        }
    }
    std::cout << "Goodbye!\n";
    return 0;
}
