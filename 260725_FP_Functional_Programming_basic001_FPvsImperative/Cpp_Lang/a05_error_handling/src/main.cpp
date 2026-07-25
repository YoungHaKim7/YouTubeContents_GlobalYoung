// Error Handling in Functional Programming Style
#include <algorithm>
#include <cmath>
#include <expected>
#include <functional>
#include <optional>
#include <print>
#include <ranges>
#include <string>
#include <vector>

// Error types for std::expected
enum class MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
    EmptyInput
};

// Custom error category for error messages
const char *to_string(MathError error) {
    switch (error) {
    case MathError::DivisionByZero:
        return "Division by zero";
    case MathError::NegativeSquareRoot:
        return "Negative square root";
    case MathError::Overflow:
        return "Arithmetic overflow";
    case MathError::EmptyInput:
        return "Empty input";
    default:
        return "Unknown error";
    }
}

// Using std::optional for nullable values
std::optional<int> parse_number(const std::string &str) {
    try {
        return std::stoi(str);
    } catch (...) {
        return std::nullopt;
    }
}

// Using std::expected for operations with specific error types
std::expected<double, MathError> safe_divide(double a, double b) {
    if (b == 0.0) {
        return std::unexpected(MathError::DivisionByZero);
    }
    if (std::abs(a) > 1e308 || std::abs(b) > 1e308) {
        return std::unexpected(MathError::Overflow);
    }
    return a / b;
}

std::expected<double, MathError> safe_sqrt(double x) {
    if (x < 0.0) {
        return std::unexpected(MathError::NegativeSquareRoot);
    }
    return std::sqrt(x);
}

std::expected<double, MathError>
calculate_statistics(const std::vector<double> &values) {
    if (values.empty()) {
        return std::unexpected(MathError::EmptyInput);
    }

    auto sum = std::ranges::fold_left(values, 0.0, std::plus{});
    return sum / values.size();
}

// Functional chaining with std::expected (C++23 monadic operations)
std::expected<double, MathError> process_value(double input) {
    return safe_divide(100.0, input) // divide 100 by input
        .and_then([](double result) {
            return safe_sqrt(result); // take square root
        })
        .transform([](double result) {
            return result * 2.0; // multiply by 2
        });
}

// Optional chaining (C++23 monadic operations)
std::optional<int> process_optional(const std::string &str) {
    return parse_number(str)
        .and_then([](int value) -> std::optional<int> {
            if (value < 0)
                return std::nullopt; // reject negative numbers
            return value;
        })
        .transform([](int value) {
            return value * 2; // double the value
        });
}

int main() {
    std::println("=== Functional Error Handling Demo ===\n");

    // 1. std::optional examples
    std::println("1. std::optional (nullable values):");

    std::vector<std::string> inputs = {"42", "100", "abc", "-50", "200"};

    for (const auto &str : inputs) {
        auto result = process_optional(str);
        if (result) {
            std::println("  '{}' -> {}", str, *result);
        } else {
            std::println("  '{}' -> failed (invalid or negative)", str);
        }
    }

    // 2. std::expected examples
    std::println("\n2. std::expected (operations with errors):");

    std::vector<double> test_values = {4.0, 0.0, -4.0, 100.0};

    for (auto value : test_values) {
        auto result = process_value(value);
        if (result) {
            std::println("  process_value({}) -> {}", value, *result);
        } else {
            std::println("  process_value({}) -> Error: {}", value,
                         to_string(result.error()));
        }
    }

    // 3. Statistics calculation
    std::println("\n3. Statistics calculation:");

    std::vector<std::vector<double>> datasets = {{1.0, 2.0, 3.0, 4.0, 5.0},
                                                 {}, // empty dataset
                                                 {10.0, 20.0, 30.0}};

    for (const auto &data : datasets) {
        auto result = calculate_statistics(data);
        if (result) {
            std::println("  Dataset size: {} -> Average: {}", data.size(),
                         *result);
        } else {
            std::println("  Dataset size: {} -> Error: {}", data.size(),
                         to_string(result.error()));
        }
    }

    // 4. Functional pipeline with error handling
    std::println("\n4. Functional pipeline:");

    auto pipeline = [](double start) -> std::expected<double, MathError> {
        return safe_divide(start, 2.0)
            .and_then([](double x) { return safe_sqrt(x); })
            .and_then([](double x) { return safe_divide(x, 3.0); })
            .transform([](double x) { return x * x; });
    };

    std::vector<double> pipeline_inputs = {36.0, 0.0, -9.0};
    for (auto input : pipeline_inputs) {
        auto result = pipeline(input);
        if (result) {
            std::println("  Pipeline({}) -> {}", input, *result);
        } else {
            std::println("  Pipeline({}) -> Error: {}", input,
                         to_string(result.error()));
        }
    }

    std::println("\n✓ All error handling examples completed successfully!");
}
