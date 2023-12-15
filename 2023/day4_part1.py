class Card:
    def __init__(self, winners: list[int], entries: list[int]) -> None:
        self.winning_numbers = winners
        self.card_entries = entries

def read_input(path: str) -> list[Card]:
    card_list = list()
    
    with open(path, "r") as f:
        for line in f:
            data = line.strip("\n")
            
            # This is a more robust way of omitting the stuff before the colon
            # than using indices because the card number might have multiple
            # digits.
            colon_idx = line.find(":")
            
            number_data = data[colon_idx + 1:]
            
            winning_string, entries_string = number_data.split("|")
            
            winning_list = list(map(int, winning_string.split()))
            entries_list = list(map(int, entries_string.split()))
            
            card_list.append(Card(winning_list, entries_list))
    
    return card_list

def calc_points(card: Card) -> int:
    hits = list(filter(lambda n: n in card.winning_numbers, card.card_entries))
    
    # We calculate the points based on the number of `card_entries` numbers that
    # are also present in the `winning_numbers`` list. If there are none, then
    # `len(hits)` is 0 and the corresponding points for that card is 0.
    if len(hits) == 0:
        return 0
    else:
        # If there are `n` hits, then the formula for the number of points is 2^(n - 1).
        return 2 ** (len(hits) - 1)

if __name__ == "__main__":
    card_list = read_input("inputs/day4-test.txt")
    
    print(sum(map(calc_points, card_list)))