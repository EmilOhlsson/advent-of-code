#include <cstdint>
#include <fmt/core.h>
#include <ranges>

#include "input.hpp"

/* As to allow constexpr */
static constexpr bool is_digit(char ch) {
    switch (ch) {
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
        case '8':
        case '9':
            return true;
        default:
            return false;
    }
}

static constexpr uint64_t calib_number(std::string_view line) {
    auto first = std::ranges::find_if(line, is_digit);
    auto last = std::ranges::find_if(std::ranges::reverse_view(line), is_digit);

    return (static_cast<uint64_t>(*first) - '0') * 10 + (static_cast<uint64_t>(*last) - '0');
}

static constexpr uint64_t solve(std::string_view text) {
    size_t pos = 0;
    size_t new_pos;
    uint64_t sum = 0;
    while ((new_pos = text.find("\n", pos)) != text.npos) {
        std::string_view line = text.substr(pos, new_pos - pos);
        sum += calib_number(line);
        pos = new_pos + 1;
    }

    return sum;
}

int main() {
    static constexpr uint64_t answer = solve(input);
    fmt::print("{}\n", answer);
}
