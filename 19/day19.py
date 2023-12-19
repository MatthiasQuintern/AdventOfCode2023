#!/usr/bin/env python3
import re
from sys import exit
import concurrent.futures
FILENAME = "input.txt"
# FILENAME = "example.txt"
CATEGORIES = "xmas"
# from typing import Callable
# rules: dict[str, list[Callable[[list[int]], bool|None|str]]] = {}
rules: dict[str, list[str|bool|tuple[int,str,int,str|bool]]] = {}
def make_rulename(r): return True if r == "A" else (False if r == "R" else r)

with open(FILENAME, "r") as file:
    lines = file.readlines()
empty_i = lines.index("\n")
# make rules
for i in range(0, empty_i):
    curly = lines[i].find("{")
    rulename = lines[i][:curly]
    last_comma = lines[i].rfind(",")
    catchall = make_rulename(lines[i][last_comma+1:-2])
    rules[rulename] = []
    for m in re.finditer(r"([xmas])([<>])(\d+):([a-zAR]+)", lines[i][curly+1:last_comma]):
        assert(m is not None)
        category, op, n, rule = m.groups()
        # print(f"\t{CATEGORIES[category]}({category}){op}{int(n)}:{rule}")
        rules[rulename].append((CATEGORIES.find(category), op, int(n), make_rulename(rule) ))
        # if op == ">": rules[rulename].append(lambda cats,c=category,n=int(n),res=rule: res if cats[c] > n else None)
        # else:         rules[rulename].append(lambda cats,c=category,n=int(n),res=rule: res if cats[c] < n else None)
    rules[rulename].append(catchall)


def apply_rules(cats: list[int], rulename) -> bool|str:
    # print(f"{rulename} -> ", end="")
    for rule in rules[rulename]:
        if type(rule) == tuple:
            c, op, n, rule = rule
            if op == ">":   res = rule if cats[c] > n else None
            else:           res = rule if cats[c] < n else None
        else:
            res = rule
        assert(res != rulename)
        if res is not None: return res
    print("Error: rules ran out")
    exit(1)

def is_accepted(values):
    res = apply_rules(values, "in")
    while type(res) != bool:
        res = apply_rules(values, res)
    return res


total_ratings = 0
for i in range(empty_i+1, len(lines)):
    values = [ int(m.group()) for m in re.finditer(r"\d+", lines[i])]
    if is_accepted(values):
        total_ratings += sum(values)
print(f"Sum of ratings of accepted parts: {total_ratings}")

splits = [[1, 4001] for _ in range(4)]
for rules_list in rules.values():
    for r in rules_list:
        if type(r) == tuple:
            c, op, n, rule = r
            # splits[c].append(n)
            splits[c].append(n + 1 if op == ">" else n)
            # x < N 
for i in range(4):
    splits[i].sort()
    # print(splits[i])

def get_acc_comb_count(sp:list[list[int]]):
    n_accepted = 0
    assert(len(sp[0]) > 1)
    for x in range(0, len(sp[0])-1):
        xdiff = sp[0][x+1] - sp[0][x]
        # print(f"{(x * 100)//len(sp[0]):3}%, x={x}", end="\r")
        for m in range(0, len(sp[1])-1):
            mdiff = sp[1][m+1] - sp[1][m]
            for a in range(0, len(sp[2])-1):
                adiff = sp[2][a+1] - sp[2][a]
                for s in range(0, len(sp[3])-1):
                    sdiff = sp[3][s+1] - sp[3][s]
                    v = [sp[0][x], sp[1][m], sp[2][a], sp[3][s]]
                    if is_accepted(v):
                        n_accepted += xdiff * mdiff * adiff * sdiff

    return n_accepted

def split_list(l, n):
    new_lists = []
    i = 1
    n = max(len(l) // n, 1)
    while i < len(l):
        new_lists.append(l[i-1:min(i+n, len(l))])
        i += n
    return new_lists

n_threads = 16
with concurrent.futures.ThreadPoolExecutor(max_workers=n_threads) as executor:
    it = executor.map(get_acc_comb_count, ([s, splits[1], splits[2], splits[3]] for s in split_list(splits[0], n_threads)))
n_accepted = 0
for n in it:
    n_accepted += n


print(f"Number of accepted combinations: {n_accepted}")



