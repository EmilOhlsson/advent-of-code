#include <cstdint>
#include <fmt/core.h>
#include <ranges>

#include "input.hpp"
#include "utils.hpp"

/* As to allow constexpr */
static constexpr bool is_digit(char ch) {
    return std::isdigit(ch);
}

static constexpr uint64_t calib_number_p1(std::string_view line) {
    auto first = std::ranges::find_if(line, is_digit);
    auto last = std::ranges::find_if(std::ranges::reverse_view(line), is_digit);

    return (static_cast<uint64_t>(*first) - '0') * 10 + (static_cast<uint64_t>(*last) - '0');
}

static constexpr uint64_t calib_number_p2(std::string_view line) {
    static constexpr std::array<std::tuple<const char *, uint64_t>, 10> numbers{
        std::tuple{"zero", 0},  std::tuple{"one", 1},  std::tuple{"two", 2}, std::tuple{"three", 3},
        std::tuple{"four", 4},  std::tuple{"five", 5}, std::tuple{"six", 6}, std::tuple{"seven", 7},
        std::tuple{"eight", 8}, std::tuple{"nine", 9},
    };
    /* Find first */
    std::optional<uint64_t> first;
    for (size_t pos = 0; pos < line.length(); pos++) {
        std::string_view tmp_line{line.substr(pos)};
        if (char ch = tmp_line[0]; is_digit(ch)) {
            first = ch - '0';
        } else {
            for (const auto &[str, val] : numbers) {
                if (tmp_line.starts_with(str)) {
                    first = val;
                    break;
                }
            }
        }
        if (first.has_value()) { break; }
    }

    std::optional<uint64_t> second;
    for (size_t length = line.length(); length > 0; length--) {
        std::string_view tmp_line{line.substr(0, length)};
        if (char ch = tmp_line.back(); is_digit(ch)) {
            second = ch - '0';
        } else {
            for (const auto &[str, val] : numbers) {
                if (tmp_line.ends_with(str)) {
                    second = val;
                    break;
                }
            }
        }
        if (second.has_value()) { break; }
    }

    return *first * 10 + *second;
}

static constexpr std::tuple<uint64_t, uint64_t> solve(std::string_view text) {
    size_t pos = 0;
    size_t new_pos;
    uint64_t sum_p1 = 0;
    uint64_t sum_p2 = 0;
    while ((new_pos = text.find("\n", pos)) != text.npos) {
        std::string_view line = text.substr(pos, new_pos - pos);
        sum_p1 += calib_number_p1(line);
        sum_p2 += calib_number_p2(line);
        pos = new_pos + 1;
    }

    return {sum_p1, sum_p2};
}

int main() {
    static constexpr std::tuple<uint64_t, uint64_t> answers{solve(input)};
    std::puts(string_rep<0>::value);
    std::puts(string_rep<std::get<0>(answers)>::value);
    std::puts(string_rep<std::get<1>(answers)>::value);
}
