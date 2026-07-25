// Functional Iterator Demo in C++23
#include <algorithm>
#include <cmath>
#include <concepts>
#include <expected>
#include <functional>
#include <iterator>
#include <numeric>
#include <optional>
#include <print>
#include <ranges>
#include <string>
#include <vector>

// Error types for functional operations
enum class MathError { DivisionByZero, NegativeSquareRoot, EmptyInput };

std::string to_string(MathError error) {
    switch (error) {
    case MathError::DivisionByZero:
        return "Division by zero";
    case MathError::NegativeSquareRoot:
        return "Negative square root";
    case MathError::EmptyInput:
        return "Empty input";
    }
    return "Unknown error";
}

// Functional pipeline using std::expected for error handling
using PipelineResult = std::expected<double, MathError>;

PipelineResult square_root(double x) {
    if (x < 0) {
        return std::unexpected(MathError::NegativeSquareRoot);
    }
    return std::sqrt(x);
}

PipelineResult divide(double x, double y) {
    if (y == 0) {
        return std::unexpected(MathError::DivisionByZero);
    }
    return x / y;
}

PipelineResult pipeline(double input) {
    // Chain: sqrt(input) -> divide(sqrt, 2) -> divide(result, 3)
    auto step1 = square_root(input);
    if (!step1) {
        return step1;
    }

    auto step2 = divide(*step1, 2.0);
    if (!step2) {
        return step2;
    }

    return divide(*step2, 3.0);
}

// Custom functional iterator using C++23 ranges
template <std::ranges::input_range R>
auto functional_filter_transform(R &&range, auto pred, auto transform) {
    return std::views::filter(std::forward<R>(range), pred) |
           std::views::transform(transform);
}

// Statistics calculator using functional style
struct Statistics {
    double mean;
    double sum;
    size_t count;
};

std::expected<Statistics, MathError>
calculate_statistics(const std::vector<int> &data) {
    if (data.empty()) {
        return std::unexpected(MathError::EmptyInput);
    }

    auto sum = std::accumulate(data.begin(), data.end(), 0.0);
    return Statistics{sum / data.size(), sum, data.size()};
}

// Custom iterator class for functional operations
template <typename T> class FunctionalIterator {
  public:
    using iterator_category = std::input_iterator_tag;
    using value_type = T;
    using difference_type = std::ptrdiff_t;
    using pointer = T *;
    using reference = T &;

    FunctionalIterator() : current_(T{}), has_value_(false) {}

    FunctionalIterator(T start) : current_(start), has_value_(true) {}

    reference operator*() { return current_; }
    pointer operator->() { return &current_; }

    FunctionalIterator &operator++() {
        if (has_value_) {
            ++current_;
        }
        return *this;
    }

    FunctionalIterator operator++(int) {
        FunctionalIterator temp = *this;
        ++(*this);
        return temp;
    }

    bool operator==(const FunctionalIterator &other) const {
        return has_value_ == other.has_value_ &&
               (!has_value_ || current_ == other.current_);
    }

  private:
    T current_;
    bool has_value_;
};

// Range generator using iterator
template <typename T> class FunctionalRange {
  public:
    FunctionalRange(T begin, T end) : begin_(begin), end_(end) {}

    FunctionalIterator<T> begin() { return FunctionalIterator<T>(begin_); }
    FunctionalIterator<T> end() { return FunctionalIterator<T>(end_); }

  private:
    T begin_;
    T end_;
};

// Demonstrate functional composition with iterators
auto make_range = [](auto start, auto end) {
    return FunctionalRange(start, end);
};

int main() {
    std::println("=== Functional Iterator Demo ===\n");

    // 1. Custom functional iterator
    std::println("1. Custom Functional Iterator:");
    auto range = make_range(1, 6);
    for (auto i : range) {
        std::println("  Iterator value: {}", i);
    }

    // 2. Functional pipeline with error handling
    std::println("\n2. Functional pipeline with error handling:");
    std::vector<double> pipeline_inputs = {36.0, 0.0, -9.0, 100.0};
    for (auto input : pipeline_inputs) {
        auto result = pipeline(input);
        if (result) {
            std::println("  Pipeline({}) -> {}", input, *result);
        } else {
            std::println("  Pipeline({}) -> Error: {}", input,
                         to_string(result.error()));
        }
    }

    // 3. Statistics calculation using functional style
    std::println("\n3. Statistics calculation:");
    std::vector<std::vector<int>> datasets = {
        {1, 2, 3, 4, 5}, {}, {10, 20, 30}};

    for (const auto &data : datasets) {
        auto stats = calculate_statistics(data);
        if (stats) {
            std::println("  Dataset size: {} -> Mean: {:.2f}, Sum: {:.2f}",
                         stats->count, stats->mean, stats->sum);
        } else {
            std::println("  Dataset size: {} -> Error: {}", data.size(),
                         to_string(stats.error()));
        }
    }

    // 4. Functional filter and transform with ranges
    std::println("\n4. Functional filter and transform:");
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    auto result = functional_filter_transform(
        numbers, [](int n) { return n % 2 == 0; }, // filter even numbers
        [](int n) { return n * n; }                // square them
    );

    std::println("  Even numbers squared: ");
    for (auto val : result) {
        std::println("    {}", val);
    }

    // 5. Functional composition with multiple operations
    std::println("\n5. Functional composition:");
    auto composited = numbers |
                      std::views::filter([](int n) { return n > 5; }) |
                      std::views::transform([](int n) { return n * 2; }) |
                      std::views::take(3);

    std::println("  Numbers > 5, doubled, first 3: ");
    for (auto val : composited) {
        std::println("    {}", val);
    }

    // 6. Demonstrate iterator algorithms
    std::println("\n6. Iterator algorithms:");
    std::vector<int> source = {3, 1, 4, 1, 5, 9, 2, 6};

    std::println("  Original: ");
    for (auto n : source) {
        std::println("    {}", n);
    }

    // Sort using iterators
    std::ranges::sort(source);
    std::println("  Sorted: ");
    for (auto n : source) {
        std::println("    {}", n);
    }

    // Find using iterators
    auto found = std::ranges::find(source, 5);
    if (found != source.end()) {
        std::println("  Found 5 at position: {}",
                     std::distance(source.begin(), found));
    }

    std::println(
        "\n✓ All functional iterator examples completed successfully!");
    return 0;
}