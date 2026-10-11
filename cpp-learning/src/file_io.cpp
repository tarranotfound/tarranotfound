#include <fstream>
#include <iostream>
#include <string>

int main() {
    const std::string path = "notes.txt";
    { std::ofstream out(path); if (!out) { std::cerr << "Cannot write file\n"; return 1; }
      out << "Learning C++ file I/O\n"; }
    std::ifstream in(path);
    std::string line;
    while (std::getline(in, line)) std::cout << line << '\n';
}
