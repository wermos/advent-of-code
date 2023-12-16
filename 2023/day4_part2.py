class Card:
    def __init__(self, number: int, winners: list[int], entries: list[int]) -> None:
        self.id = number
        self.winning_numbers = winners
        self.card_entries = entries

def read_input(path: str) -> list[Card]:
    card_list = list()

    with open(path, "r") as f:
        for line in f:
            data = line.strip("\n")

            # This is a more robust way than using indices because the card
            # number might have multiple digits.
            colon_idx = line.find(":")

            card_number = int(data[:colon_idx].split()[1])

            number_data = data[colon_idx + 1:]

            winning_string, entries_string = number_data.split("|")

            winning_list = list(map(int, winning_string.split()))
            entries_list = list(map(int, entries_string.split()))

            card_list.append(Card(card_number, winning_list, entries_list))

    return card_list

def calculate_hits(card: Card) -> int:
    """
    Returns how many of the card's number entries were in the winning number
    list.
    """
    hits = list(filter(lambda n: n in card.winning_numbers, card.card_entries))

    return len(hits)

def calc_num_cards(cards: list[Card]) -> int:
    frequency = [1] * (len(cards) + 1) # because Python indexing starts at 0

    for card in cards:
        num_hits = calculate_hits(card)

        for i in range(1, num_hits + 1):
            # We get one copy of card number `card.id + i` for each copy of the
            # current card, and there are `frequency[card.id]` cards of the
            # current ID. So, we add `frequency[card.id]` to the frequency of
            # card number `card.id + i`.
            frequency[card.id + i] += frequency[card.id]

    # The answer to "how many cards do we have at the end" is simply the sum
    # of all the card frequencies.
    return sum(frequency[1:]) # because index 0 has some garbage value

if __name__ == "__main__":
    card_list = read_input("inputs/day4.txt")

    print(calc_num_cards(card_list))