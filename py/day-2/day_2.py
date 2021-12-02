from functools import reduce


command_to_dir = {
    "up": (-1, 0),
    "down": (1, 0),
    "forward": (0, 1),
}


def read_and_parse_input(filename):
    with open(filename) as f:
        lines = [li.strip() for li in f.readlines()]

    commands_parsed = [li.split() for li in lines]
    commands_parsed = [(command_to_dir[d], int(s)) for d, s in commands_parsed]
    commands_parsed = [(cmd[0] * step, cmd[1] * step) for cmd, step in commands_parsed]

    return commands_parsed


if __name__ == "__main__":
    commands_parsed = read_and_parse_input("input.txt")
    starting_pos = (0, 0)

    # Part 1
    def accumulator_1(current, x):
        return current[0] + x[0], current[1] + x[1]

    final_pos = reduce(accumulator_1, commands_parsed, starting_pos)
    print(final_pos, "=>", final_pos[0] * final_pos[1])

    # Part 2
    starting_pos = (0, 0, 0)

    def accumulator_1(current, x):
        aim = current[0] + x[0]
        depth = current[1] + x[1] * aim
        horizontal = current[2] + x[1]
        return aim, depth, horizontal

    final_pos = reduce(accumulator_1, commands_parsed, starting_pos)
    print(final_pos, "=>", final_pos[1] * final_pos[2])
