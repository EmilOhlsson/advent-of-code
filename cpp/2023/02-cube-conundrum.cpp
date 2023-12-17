#include <algorithm>
#include <cassert>
#include <cstdint>
#include <fmt/core.h>
#include <fmt/ranges.h>
#include <functional>
#include <ranges>
#include <span>

#include "input-02.hpp"
#include "utils.hpp"

using SV = std::string_view;
using u64 = uint64_t;
using KV = std::pair<SV, u64>;

constexpr std::vector<KV> copy_keys(std::span<const KV> kvs) {
    std::vector<KV> result;
    std::ranges::transform(kvs, std::back_inserter(result), [](const KV &kv) {
        return KV{kv.first, {}};
    });
    return result;
}

constexpr u64 solve_p1(SV input, std::span<const KV> available) {
    using std::operator""sv;
    const std::vector empty = copy_keys(available);
    u64 game_id = 1;
    u64 id_sum{};
    for (const auto line : std::views::split(input, "\n"sv)) {
        const auto commands = *(std::views::split(line, ": "sv) | std::views::drop(1)).begin();
        bool valid = true;
        for (const auto block : std::views::split(SV{commands}, "; "sv)) {
            std::vector used = empty;
            for (const auto count_color_txt : std::views::split(SV{block}, ", "sv)) {
                auto token_view = std::views::split(count_color_txt, " "sv);
                auto token_iterator = token_view.begin();
                const u64 count = *utils::parse<u64>(SV{*token_iterator});
                token_iterator++;
                const SV color{*token_iterator};

                /* Find key in `used`, and increase used-count (hash-map isn't constexpr) */
                auto item =
                    std::ranges::find_if(used, [&](const KV &kv) { return kv.first == color; });
                assert(item != end(used));
                get<1>(*item) += count;

                /* Compare with available */
                const auto avail = std::ranges::find_if(
                    available, [&](const KV &kv) { return kv.first == color; });
                assert(avail != end(available));
                if (get<1>(*item) > get<1>(*avail)) {
                    valid = false;
                    break;
                }
            }
        }
        if (valid) { id_sum += game_id; }

        game_id += 1;
    }

    return id_sum;
}
constexpr u64 solve_p2(SV input) {
    using std::operator""sv;
    u64 power_sum{};

    for (const auto line : std::views::split(input, "\n"sv)) {
        std::vector<KV> used;
        const auto commands = *(std::views::split(line, ": "sv) | std::views::drop(1)).begin();
        for (const auto block : std::views::split(SV{commands}, "; "sv)) {
            for (const auto count_color_txt : std::views::split(SV{block}, ", "sv)) {
                auto token_view = std::views::split(count_color_txt, " "sv);
                auto token_iterator = token_view.begin();
                const u64 count = *utils::parse<u64>(SV{*token_iterator});
                token_iterator++;
                const SV color{*token_iterator};

                /* Find key in `used`, and increase used-count (hash-map isn't constexpr) */
                if (auto item =
                        std::ranges::find_if(used, [&](const KV &kv) { return kv.first == color; });
                    item != end(used)) {
                    u64 prev_max = get<1>(*item);
                    get<1>(*item) = std::max<u64>(count, prev_max);
                } else {
                    used.push_back({color, count});
                }
            }
        }
        /* Multiply all counts */
        u64 power = std::ranges::fold_left_first(
                        used | std::views::transform([](const KV &kv) { return kv.second; }),
                        std::multiplies<u64>())
                        .value_or(0);
        power_sum += power;
    }
    return power_sum;
}
inline constexpr SV test_input =
    R"(Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green)";

int main() {
    using namespace std::literals;
    static constexpr std::array available{
        KV{"red"sv, 12},
        KV{"green"sv, 13},
        KV{"blue"sv, 14},
    };
    static constexpr std::span<const KV> pairs = available;
    std::puts(utils::string_rep<solve_p1(input02, pairs)>::value);
    std::puts(utils::string_rep<solve_p2(input02)>::value);
};
