import os
from collections import Counter
from dataclasses import dataclass

day = os.path.dirname(os.path.realpath(__file__))


@dataclass
class Hand:
    cards: str
    bid: int
    type: int


def main(file: str, wildcard=False):
    labels = "J23456789TQKA" if wildcard else "23456789TJQKA"

    hands = [Hand(cards, int(bid), 0) for [cards, bid] in [line.split(" ") for line in file.splitlines()]]
    for hand in hands:
        hand.type = get_hand_type(hand.cards)
        if wildcard:
            hand.type = max([get_hand_type(hand.cards.replace("J", label)) for label in labels[1:]])

    hands.sort(key=lambda hand: (hand.type, [labels.index(c) for c in hand.cards]))
    return sum(hand.bid * (i + 1) for [i, hand] in enumerate(hands))


def get_hand_type(cards: str):
    card_counts = list(Counter(cards).values())
    if 5 in card_counts:
        return 6
    elif 4 in card_counts:
        return 5
    elif 3 in card_counts and 2 in card_counts:
        return 4
    elif 3 in card_counts:
        return 3
    elif card_counts.count(2) == 2:
        return 2
    elif 2 in card_counts:
        return 1
    return 0


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(main(content)))
        print("Part 2: " + str(main(content, True)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert main(content) == 6440
        assert main(content, True) == 5905
