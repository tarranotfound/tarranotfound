#include <algorithm>
#include <iostream>
#include <string>

int main() {
    std::string text;
    std::cout << "Enter a line: ";
    std::getline(std::cin, text);
    std::transform(text.begin(), text.end(), text.begin(),
                   [](unsigned char c) { return static_cast<char>(std::toupper(c)); });
    std::cout << "Uppercase: " << text << '\n'
              << "Length: " << text.size() << '\n';
}
