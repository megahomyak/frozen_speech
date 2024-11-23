import os

def parse(text):
    items = {}
    nestlvl = 0
    name = ""
    openindex = 0
    for idx, c in enumerate(text):
        if c == "(":
            if nestlvl == 0:
                openindex = idx
            nestlvl += 1
        elif c == ")":
            nestlvl -= 1
            if nestlvl == 0:
                items[name.strip()] = text[openindex+1:idx]
                name = ""
        elif nestlvl == 0:
            name += c
    return items

print(parse("  abc (def ()) ghi ()"))

exit()

for discussion in os.listdir(discussions := "discussions"):
    messages = []
    sender = lambda name: messages.append()
    line = lambda arg: messages[-1].append("")
    exec()
    for line in open(f"{discussions}/{discussion}").read().splitlines():
        pass
