#include <iostream>
#include <sstream>
#include <string>

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

    double result{};
    switch (op) {
        case '+': result = a + b; break;
        case '-': result = a - b; break;
        case '*': result = a * b; break;
        default:
            std::cerr << "Supported operators: + - *\n";
            return 1;
    }

    std::cout << "Result: " << result << '\n';
    return 0;
}
