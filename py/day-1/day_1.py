from itertools import islice
from collections import deque


def read_data(filename):
    """Reads an input file and as a list of integers."""
    with open(filename) as f:
        data = [int(li.strip()) for li in f.readlines()[:-1]]
    return data


def rolling_sum(iterable, n=3):
    """Computes a rolling sum of an iterable over a window size `n`."""
    it = iter(iterable)
    window = deque(islice(it, n), maxlen=n)

    if len(window) == n:
        yield sum(window)
    for x in it:
        window.append(x)
        yield sum(window)


def count_increments(data):
    """Counts positive increments in a sequence."""
    inc = [next_x - x for x, next_x in zip(data[:-1], data[1:])]
    return sum(i > 0 for i in inc)


if __name__ == "__main__":
    # Part 1
    data = read_data("input.txt")
    result = count_increments(data)
    print(result)

    # Part 2
    data_window = [s for s in rolling_sum(data, n=3)]
    result = count_increments(data_window)
    print(result)
