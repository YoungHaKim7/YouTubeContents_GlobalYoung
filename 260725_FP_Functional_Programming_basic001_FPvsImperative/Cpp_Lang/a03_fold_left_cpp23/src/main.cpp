// FP style
#include <algorithm>
#include <print>
#include <ranges>

int main() {
    auto numbers = std::views::iota(1, 11); // 1..10

    int sum = std::ranges::fold_left(numbers, 0,
                                     [](int acc, int x) { return acc + x; });

    std::println("{}", sum);
}
