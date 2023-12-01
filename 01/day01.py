import re
# part 1
with open("input.txt", "r") as file: print("Part 1:", sum([int(match[0]+match[-1]) for match in [re.findall(r"\d", line) for line in file.readlines()]]))

# part 2 use lookahead in findall to get two matches one, two for "twone"
with open("input.txt", "r") as file: print(sum([int("".join([conv[0],conv[-1]])) for conv in [(lambda x : [ str(n.index(y) + 1) if y in n else y for y,n in zip(x, [['one','two','three','four','five','six','seven','eight','nine'] for _ in range(len(x))])])(re.findall(r"(?=(one|two|three|four|five|six|seven|eight|nine|\d))",line)) for line in file.readlines()]]))
