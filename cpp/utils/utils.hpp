#pragma once

#include <cctype>
#include <cstdint>
#include <functional>
#include <optional>
#include <string_view>

namespace utils {

/**
 * Implementation details of `string_rep`
 */
namespace num2sv {
/* Container for string representing digits */
template <uint64_t... digits> struct to_chars {
    static constexpr char value[] = {('0' + digits)..., 0};
};

/* Expand `remaining` to digits, */
template <uint64_t remaining, uint64_t... digits>
struct expand : expand<remaining / 10, remaining % 10, digits...> {};

/* Base case, for remaining = 0, that is done expanding number */
template <uint64_t... digits> struct expand<0, digits...> : to_chars<digits...> {};
template <> struct expand<0> : to_chars<0> {};
} // namespace num2sv

/**
 * Create a string representation of a numeric value. Uses base 10, and `uint64_t`
 */
template <uint64_t number> struct string_rep : num2sv::expand<number> {};

/**
 * Parse a string into numeric value
 */
template <typename T>
inline constexpr std::optional<T> parse(std::string_view str) {
    T result {};
    if (str.length() == 0) {
        return std::nullopt;
    }
    for (const char ch : str) {
        if (std::isdigit(ch)) {
            result = result * 10 + (ch - '0');
        } else {
            return std::nullopt;
        }
    }
    return result;
}

} // namespace utils
