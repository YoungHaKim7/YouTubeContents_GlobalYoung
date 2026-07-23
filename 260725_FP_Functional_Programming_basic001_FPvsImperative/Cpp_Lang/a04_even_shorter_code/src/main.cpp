// FP style(even shorter code)
#include <algorithm>
#include <functional>
#include <print>
#include <ranges>

int main() {
    auto numbers = std::views::iota(1, 11);

    int sum = std::ranges::fold_left(numbers, 0, std::plus{});

    std::println("{}", sum);
}
