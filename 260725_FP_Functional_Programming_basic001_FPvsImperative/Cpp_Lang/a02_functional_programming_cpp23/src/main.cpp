// FP style
#include <numeric>
#include <print>
#include <ranges>

int main() {
    auto numbers = std::views::iota(1, 11); // 1..10

    int sum = std::accumulate(numbers.begin(), numbers.end(), 0);

    std::println("{}", sum);
}
