import math

import re
from typing import List, Callable, Tuple


class Monkey:
    def __init__(self, items: List[int], divisor: int, test_true: int, test_false: int, op: Callable[[int], int]):
        self.op = op
        self.test_false = test_false
        self.test_true = test_true
        self.divisor = divisor
        self.items = items
        self.activity = 0

    def process_items(self, reducer: Callable[[int], int]) -> List[Tuple[int, int]]:
        res = []
        for item in self.items:
            self.activity += 1
            new_value = reducer(self.op(item))
            if new_value % self.divisor == 0:
                res.append((self.test_true, new_value))
            else:
                res.append((self.test_false, new_value))
        self.items = []
        return res


class Monkeys:
    def __init__(self, monkeys: List[Monkey]):
        self.monkeys = monkeys

    def round(self, reducer: Callable[[int], int]):
        for monkey in self.monkeys:
            moves = monkey.process_items(reducer)
            for (target_monkey, value) in moves:
                self.monkeys[target_monkey].items.append(value)

    def sorted_activity(self) -> List[int]:
        return sorted([m.activity for m in self.monkeys], reverse=True)


def parse_monkey(block: str) -> Monkey:
    lines = [line.strip() for line in block.split("\n")][1:]
    items = [int(item) for item in re.findall("\\d+", lines[0])]
    divisor = [int(d) for d in re.findall("\\d+", lines[2])][0]
    test_true = [int(d) for d in re.findall("\\d+", lines[3])][0]
    test_false = [int(d) for d in re.findall("\\d+", lines[4])][0]
    op = lambda old: eval(lines[1][17:])
    return Monkey(items, divisor, test_true, test_false, op)


def parse_data(data: str) -> Monkeys:
    return Monkeys([parse_monkey(block) for block in data.split("\n\n")])


def solve(monkeys: Monkeys, rounds: int, d: Callable[[int], int]) -> int:
    for _ in range(20):
        monkeys.round(d)
    activity = monkeys.sorted_activity()
    return activity[0] * activity[1]


def part1(monkeys: Monkeys) -> int:
    return solve(monkeys, 20, lambda x: x // 3)


def part2(monkeys: Monkeys) -> int:
    d = math.prod([m.divisor for m in monkeys.monkeys])
    return solve(monkeys, 20, lambda x: x % d)


def main():
    data = open("11.txt", 'rb').read().decode()
    print(part1(parse_data(data)))
    print(part2(parse_data(data)))


main()
