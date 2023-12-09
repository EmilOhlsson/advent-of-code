#pragma once

#include <cstdint>

namespace num2sv {
    template <uint64_t... digits>
    struct to_string_view {
        static constexpr char value[] = {('0' + digits)..., 0};
    };
    
    /* Expand `remaining` to digits, */
    template <uint64_t remaining, uint64_t... digits>
    struct expand : expand <remaining / 10, remaining % 10, digits...> {};
    
    /* Specialzied for remaining = 0 */
    template <uint64_t... digits>
    struct expand<0, digits...> : to_string_view<digits...> {};
}

template <uint64_t number>
struct string_rep : num2sv::expand<number> {};
