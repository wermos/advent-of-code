#include <cstdint>
#include <ranges>

#include <iostream>

extern "C" int calculate_ways(std::uint64_t t_max, std::uint64_t distance) {
    auto filter = [t_max, distance](std::uint64_t t) -> bool { return t * (t_max - t) >= distance; };
    // auto filter = [t_max, distance](auto t) -> bool { return t * (t_max - t) >= distance; };

    auto filteredView = std::views::iota(1) | std::views::take(t_max) | std::views::filter(filter);

    return std::ranges::distance(filteredView);
}

int calculate_ways_loop(std::uint64_t t_max, std::uint64_t distance) {
    int count = 0;

    for (std::uint64_t t = 1; t < t_max; t++) {
        // std::uint64_t d = t * (t_max - t);

        if (t * (t_max - t) >= distance) {
            count++;
        }
    }
    
    return count;
}

int main() {
    std::cout << calculate_ways(47707566, 282107911471062) << std::endl;
    // std::cout << calculate_ways_loop(47707566, 282107911471062) << std::endl;
}