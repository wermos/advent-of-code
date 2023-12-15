NUM_RED = 12
NUM_GREEN = 13
NUM_BLUE = 14


class Round:
    """
    Stores the number of red, green, and blue balls that were present in one
    round of the game.

    A round `r_1` is less than another round `r_2` if the number of red, blue,
    and green balls in `r_1` is less than or equal to the number of red, blue,
    and green balls in `r_2`.
    """
    def __init__(self, red: int, blue: int, green: int):
        self.red: int = red
        self.green: int = green
        self.blue: int = blue

    def __ge__(self, other) -> bool:
        return not (self <= other)

    def __le__(self, other) -> bool:
        return self.red <= other.red and self.green <= other.green and self.blue <= other.blue

    def __repr__(self) -> str:
        return f"Red: {self.red}, Green: {self.green}, Blue: {self.blue}"

class Game:
    """
    Stores the list of rounds that happened in the game.

    A game `g` is less than a round `r` if the number of red, blue, and green
    balls in each round `g_r` of game `g` is less than or equal to the number
    of red, blue, and green balls in `r`.
    """
    def __init__(self, game_id) -> None:
        self.game_id: int = game_id
        self.round_list: list[Round] = list()

    def append_round(self, Round: Round) -> None:
        self.round_list.append(Round)

    def __ge__(self, other: Round) -> bool:
        return not (self <= other)

    def __le__(self, other: Round) -> bool:
        return all(map(lambda round: round <= other, self.round_list))

    def __repr__(self) -> str:
        return "; ".join([str(round) for round in self.round_list])


def round_parser(line: str) -> Round:
    """
    Parses each individual round of a game and returns the result in a `Round`
    object.
    """
    data = line.split(",")

    red = 0
    green = 0
    blue = 0

    for datum in data:
        num, color_string = datum.split()

        if color_string == "red":
            red = int(num)
        elif color_string == "green":
            green = int(num)
        elif color_string == "blue":
            blue = int(num)

    return Round(red, blue, green)

def game_parser(line: str) -> Game:
    """Parses an individual game, which is a single line in the input file."""

    colon_idx = line.find(":")

    # The first five characters in every line are "Game ", so we skip them
    game_id = int(line[5:colon_idx])

    game: Game = Game(game_id)

    round_data = line[colon_idx + 1:].split(";")

    for elem in round_data:
        # We don't want any leading or trailing whitespace, nor any trailing newlines
        processed_elem = elem.strip().strip("\n")

        game.append_round(round_parser(processed_elem))

    return game

def input_parser(path: str) -> list[Game]:
    """Parses the entire input text file."""

    game_list = list()

    with open(path, "r") as f:
        for line in f:
            game_list.append(game_parser(line))

    return game_list


if __name__ == "__main__":
    game_list = input_parser("inputs/day2.txt")

    reference_round = Round(NUM_RED, NUM_BLUE, NUM_GREEN)

    id_total = 0

    for game in game_list:
        # We simply check if each round in `game` is less than or equal to the
        # `reference_round`, and if it, we add its game ID to the running total
        # of the game IDs.
        if game <= reference_round:
            id_total += game.game_id

    print("Total =", id_total)