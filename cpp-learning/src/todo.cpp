#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> tasks;
    std::string command;
    std::cout << "Commands: add, list, quit\n";
    while (true) {
        std::cout << "> ";
        if (!(std::cin >> command)) break;
        if (command == "quit") break;
        if (command == "add") {
            std::string task;
            std::getline(std::cin >> std::ws, task);
            if (!task.empty()) { tasks.push_back(task); std::cout << "Added\n"; }
        } else if (command == "list") {
            if (tasks.empty()) std::cout << "No tasks yet\n";
            for (std::size_t i = 0; i < tasks.size(); ++i)
                std::cout << i + 1 << ". " << tasks[i] << '\n';
        } else std::cout << "Unknown command\n";
    }
}
