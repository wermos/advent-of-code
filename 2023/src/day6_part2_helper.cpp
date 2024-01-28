#include "day6_part2_helper.hpp"

#include <cstdint>
#include <iostream>
#include <ranges>

extern "C" int calculate_ways(std::uint64_t t_max, std::uint64_t distance) {
    return calculate_ways_impl(t_max, distance);
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