NUM_RED = 12
NUM_GREEN = 13
NUM_BLUE = 14


class Round:
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
    """Parses the input text file."""

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
        if game <= reference_round:
            id_total += game.game_id

    print("Total =", id_total)