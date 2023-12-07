from collections import Counter


def parse_hand(line):
    translation_table = str.maketrans("AKQJT98765432", "MLKJIHGFEDCBA")
    line = line.split()[0].translate(translation_table) + " " + line.split()[1]
    hand = line.split()[0]  # order = A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
    cards = len(set(hand))  # order = M  L  K  J  I  H  G  F  E  D  C  B  A
    if cards == 1:
        return "6" + line
    if cards == 2:
        for _ in hand:
            if hand.count(_) == 4:
                return "5" + line
        return "4" + line
    if cards == 3:
        for _ in hand:
            if hand.count(_) == 3:
                return "3" + line
        return "2" + line
    if cards == 4:
        return "1" + line
    return "0" + line


def parse_hand2(line):
    def changeJ(hand):
        top = Counter(hand).most_common()
        if len(top) == 1:
            return hand
        hand = (
            hand.replace("A", top[0][0])
            if top[0][0] != "A"
            else hand.replace("A", top[1][0])
        )
        return hand

    translation_table = str.maketrans("AKQT98765432J", "MLKJIHGFEDCBA")
    line = line.split()[0].translate(translation_table) + " " + line.split()[1]
    hand = line.split()[0]  # order = A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J
    cards = len(set(hand))  # order = M  L  K  J  I  H  G  F  E  D  C  B  A
    if "A" in hand:
        cards = cards - 1
        hand = changeJ(hand)
        cards = 1 if cards == 0 else cards
    if cards == 1:
        return "6" + line
    if cards == 2:
        for _ in hand:
            if hand.count(_) == 4:
                return "5" + line
        return "4" + line
    if cards == 3:
        for _ in hand:
            if hand.count(_) == 3:
                return "3" + line
        return "2" + line
    if cards == 4:
        return "1" + line
    return "0" + line


lines = open("./inputs/day7.txt").readlines()
hands = []
p1 = 0
for line in lines:
    hands.append(parse_hand(line.strip()))
hands.sort()
for i, _ in enumerate(hands):
    p1 += int(_.split()[1]) * (i + 1)

print(f"Part 1: {p1}")

hands2 = []
p2 = 0
for line in lines:
    hands2.append(parse_hand2(line.strip()))
hands2.sort()
for i, _ in enumerate(hands2):
    p2 += int(_.split()[1]) * (i + 1)

print(f"Part 2: {p2}")
