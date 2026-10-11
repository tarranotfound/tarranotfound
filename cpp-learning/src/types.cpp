#include <iostream>
#include <string>

int main() {
    const std::string language = "C++";
    int lessons = 13;
    double progress = 0.25;
    bool learning = true;

    std::cout << language << " lessons: " << lessons << '\n'
              << "Progress: " << progress * 100 << "%\n"
              << std::boolalpha << "Learning: " << learning << '\n';
}
