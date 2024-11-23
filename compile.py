import os
import glob

path = os.path.join

for file in glob.glob(f"{(compiled_dir:='compiled')}/*.html"):
    os.remove(file)
try: os.remove(path(compiled_dir, compiled_index_name:="index.html"))
except: pass

def attrify(listobj):
    obj = dict(listobj)
    class A:
        def __getattr__(self, k): return obj.__getitem__(k)
        def __setattr__(self, k, v): obj.__setitem__(k, v)
        def _obj(self): return obj
    return A()

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
    return open(path(*path_parts), encoding="utf-8").read()

def write(text, *path_parts):
    return open(path(*path_parts), encoding="utf-8").write(text)

def templated(filepath, args):
    result = ""
    def part(arg, end="\n"):
        result += str(arg) + end
    locals_ = {"_": part, "attrify": attrify, **args}
    exec(read(filepath), {}, locals_)
    return result

def write_templated(inpath, outpath, args):
    result = templated(inpath, args)
    write(result, compiled_dir, outpath)

discussions = []
for discussion_fname in os.listdir(discussions_dir := "discussions"):
    discussion = attrify(parse(read(discussions_dir, discussion_fname)))
    participants = set()
    discussion.filename = discussion_fname
    print(discussion._obj())
    for sender_name, _ in discussion.messages:
        assert os.path.exists(path("participants", sender_name))
        participants.add(sender_name)
    discussion.participants = participants
    discussions.append(discussions)
    write_templated("discussion.html.template", {"discussions": discussions}, discussion_fname)
write_templated("index.html.template", {"discussions": discussions}, compiled_index_name)
