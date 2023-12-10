input = open("inputs/day10.txt").read().splitlines()
pipes, start = dict(), None
for i, l in enumerate(input):
    for j, c in enumerate(l):
        if c == "S":
            start = (i, j)
        elif c != ".":
            pipes[i, j] = c


def pipe(c, i, j):
    match c:
        case "-":
            return (i, j - 1), (i, j + 1)
        case "|":
            return (i - 1, j), (i + 1, j)
        case "L":
            return (i - 1, j), (i, j + 1)
        case "J":
            return (i - 1, j), (i, j - 1)
        case "7":
            return (i + 1, j), (i, j - 1)
        case "F":
            return (i + 1, j), (i, j + 1)
        case _:
            assert 0


def connected_to(pt):
    return set(pipe(pipes[*pt], *pt))


endpt1, endpt2 = [
    pt
    for di, dj in ((-1, 0), (0, 1), (0, -1), (1, 0))
    if (pt := (start[0] + di, start[1] + dj)) in pipes and start in connected_to(pt)
]

for c in "-LJ7F|":
    if set(pipe(c, *start)) == set([endpt1, endpt2]):
        pipes[start] = c

visited, steps = set([start, endpt1, endpt2]), 1
while endpt1 != endpt2:
    endpt1 = (connected_to(endpt1) - visited).pop()
    endpt2 = (connected_to(endpt2) - visited).pop()
    visited |= set([endpt1, endpt2])
    steps += 1
print(steps)

enclosed = 0
for i in range(len(input)):
    inside = False
    for j in range(len(input[0])):
        if (i, j) in visited:
            if pipes[i, j] in "|LJ":
                inside = not inside
            continue
        if inside:
            enclosed += 1
print(enclosed)
