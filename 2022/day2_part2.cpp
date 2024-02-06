#include <fstream>
#include <iostream>
#include <string>

int calculate_points(char opponent_move, char outcome) {
    /**
     * In `opponent_move`,
     *  'A' is for Rock,
     *  'B' is for Paper, and
     *  'C' is for Scissors.
     *
     * Points values are as follows:
     *  - Rock — 1,
     *  - Paper — 2, and
     *  - Scissors — 3.
     */
    int points;

    if (outcome == 'X') {
        // I need to lose
        points = 0;  // 0 points for a loss

        if (opponent_move == 'A') {
            points += 3;
        } else if (opponent_move == 'B') {
            points += 1;
        } else {
            points += 2;
        }
    } else if (outcome == 'Y') {
        // I need to draw
        points = 3;  // 0 points for a draw

        if (opponent_move == 'A') {
            points += 1;
        } else if (opponent_move == 'B') {
            points += 2;
        } else {
            points += 3;
        }
    } else {
        // I need to win
        points = 6;  // 0 points for a win

        if (opponent_move == 'A') {
            points += 2;
        } else if (opponent_move == 'B') {
            points += 3;
        } else {
            points += 1;
        }
    }

    return points;
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