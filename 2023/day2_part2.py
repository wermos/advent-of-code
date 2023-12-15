from day2_part1 import input_parser

if __name__ == "__main__":
    game_list = input_parser("inputs/day2.txt")

    total = 0

    for game in game_list:
        reds = [round.red for round in game.round_list]
        greens = [round.green for round in game.round_list]
        blues = [round.blue for round in game.round_list]

        total += max(reds) * max(greens) * max(blues)

    print(total)
