import re
import z3
from z3 import Solver


def part2(data):
    s = Solver()
    a, op, b = data['root']
    data['root'] = a, "==", b
    humn = z3.Int("humn")
    configured = {"humn": humn}
    while 'root' not in configured:
        for name, (a, op, b) in data.items():
            if name not in configured:
                if op is None:
                    x = z3.Int(name)
                    s.add(x == int(a))
                    configured[name] = x
                elif op is not None and a in configured and b in configured:
                    x = configured[a]
                    y = configured[b]
                    z = z3.Int(name)
                    configured[name] = z
                    s.add(eval(f"z == (x {op} y)", {"z": z, "x": x, "y": y}))
    s.add(configured['root'] == 1)
    print(s.check())
    return s.model()[configured['humn']]


def part1(data):
    context = {}
    while 'root' not in context:
        for name, (a, op, b) in data.items():
            try:
                if op is None:
                    context[name] = int(a)
                elif name not in context and a in context and b in context:
                    context[name] = eval(a + op + b, context)
            except:
                pass
    return int(context['root'])


def split_expression(expr):
    if expr.strip().isdigit():
        return expr, None, None
    return re.findall("(.*) ([\-+/*]) (.*)", expr)[0]


def main():
    content = open("21.txt", 'rb').read().decode()
    data = [c.split(": ") for c in content.splitlines()]
    data = {c[0]: split_expression(c[1]) for c in data}
    print(part1(data))
    print(part2(data))


main()
