import os

def parse(text):
    items = []
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
                items.append((name.strip(), text[openindex+1:idx]))
                name = ""
        elif nestlvl == 0:
            name += c
    return items

def read(*path_parts):
    import os
    return open(os.path.join(path_parts)).read()

def templated(filepath, args):
    result = ""
    def part(arg):
        result += str(arg)
    locals_ = {"part": part, **args}
    exec(read(filepath), {}, locals_)
    return result

class Template:
    def __init__(self):
        self.text = ""
    def part(self, text):
        self.text += text

for discussion in os.listdir(discussions := "discussions"):
    discussion = dict(parse(read(discussions, discussion)))
