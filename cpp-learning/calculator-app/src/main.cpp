#include <cmath>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

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
    std::cout << "Commands: help, history, clear, quit\n"
              << "Expressions: 8 + 3, 12 / 4, 2 ^ 8, sqrt 81\n";
}

int main() {
    std::cout << std::fixed << std::setprecision(4);
    std::cout << "C++ Terminal Calculator\nType help for usage.\n";
    std::vector<std::string> history;
    std::string line;

    while (true) {
        std::cout << "> ";
        if (!std::getline(std::cin, line)) break;
        if (line == "quit" || line == "exit") break;
        if (line == "help") { print_help(); continue; }
        if (line == "clear") {
            history.clear();
            std::cout << "History cleared.\n";
            continue;
        }
        if (line == "history") {
            if (history.empty()) std::cout << "History is empty.\n";
            for (const auto& entry : history) std::cout << entry << '\n';
            continue;
        }
        if (line.empty()) continue;

        std::istringstream input(line);
        std::string first;
        input >> first;

        try {
            double result{};
            if (first == "sqrt") {
                double value{};
                std::string extra;
                if (!(input >> value) || (input >> extra))
                    throw std::runtime_error("usage: sqrt 81");
                result = square_root(value);
            } else {
                std::istringstream expression(line);
                double a{}, b{};
                char op{};
                std::string extra;
                if (!(expression >> a >> op >> b) || (expression >> extra))
                    throw std::runtime_error("invalid expression; type help");
                result = calculate(a, op, b);
            }

            std::ostringstream formatted;
            formatted << line << " = " << result;
            history.push_back(formatted.str());
            std::cout << "= " << result << '\n';
        } catch (const std::exception& error) {
            std::cout << "Error: " << error.what() << '\n';
        }
    }
    std::cout << "Goodbye!\n";
    return 0;
}
