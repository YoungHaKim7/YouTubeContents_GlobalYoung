// FP sytle
#include <fstream>
#include <iterator>
#include <numeric>
#include <print>
#include <string>
#include <vector>

int count_lines(const std::string &filename) {
    std::ifstream file(filename);

    return std::accumulate(
        std::istream_iterator<std::string>(file),
        std::istream_iterator<std::string>{}, 0,
        [](int count, const std::string &) { return count + 1; });
}

int main() {
    const std::vector<std::string> files = {"a.txt", "b.txt"};

    const int total_lines = std::accumulate(
        files.begin(), files.end(), 0, [](int total, const std::string &file) {
            return total + count_lines(file);
        });

    std::println("total lines : {} .", total_lines);
}
