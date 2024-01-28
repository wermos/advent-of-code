#include <fstream>
#include <iostream>
#include <string>

int calculate_points(char opponent_move, char my_move) {
    /**
     * In `my_move`,
     *  'X' is for Rock,
     *  'Y' is for Paper, and
     *  'Z' is for Scissors.
     *
     * Similarlyk, in `opponent_move`,
     *  'A' is for Rock,
     *  'B' is for Paper, and
     *  'C' is for Scissors.
     */
    int total_points;

    if (my_move == 'X') {
        total_points = 1;
    } else if (my_move == 'Y') {
        total_points = 2;
    } else {
        total_points = 3;
    }

    if ((my_move == 'X' && opponent_move == 'C') || (my_move == 'Y' && opponent_move == 'A') ||
        (my_move == 'Z' && opponent_move == 'B')) {

        // it's a win for me
        total_points += 6;
    } else if ((my_move == 'X' && opponent_move == 'A') || (my_move == 'Y' && opponent_move == 'B') ||
               (my_move == 'Z' && opponent_move == 'C')) {

        // it's a draw
        total_points += 3;
    }

    // otherwise I gain no additional points

    return total_points;
}

int main() {
    std::ifstream input{"inputs/day2.txt"};

    int total_points = 0;

    std::string line;

    while (std::getline(input, line)) {
        // We abuse the standardized input format. Each line is 3 characters
        // long, with the first char being the opponent's move, and the last
        // char being my move.
        total_points += calculate_points(line[0], line[2]);
    }

    std::cout << total_points << std::endl;
}