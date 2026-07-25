// FP sytle
#include <fstream>
#include <print>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> files = {"a.txt", "b.txt"};
    int total_lines = 0;
    for (const auto &file : files) {
        std::ifstream f(file);
        std::string line;
        while (std::getline(f, line)) {
            total_lines++;
        }
    }

    std::println("total lines : {} .", total_lines);
}
