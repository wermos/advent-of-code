#pragma once

#include <cstdint>

int calculate_ways_impl(std::uint64_t t_max, std::uint64_t distance) {
    auto filter = [t_max, distance](std::uint64_t t) -> bool { return t * (t_max - t) >= distance; };
    // auto filter = [t_max, distance](auto t) -> bool { return t * (t_max - t) >= distance; };

    auto filteredView = std::views::iota(1) | std::views::take(t_max) | std::views::filter(filter);

    return std::ranges::distance(filteredView);
}