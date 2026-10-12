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
    if (input >> a >> op >> b) {
        std::cout << "You entered: " << a << ' ' << op << ' ' << b << '\n';
    } else {
        std::cout << "Could not read that expression.\n";
    }
    return 0;
}
