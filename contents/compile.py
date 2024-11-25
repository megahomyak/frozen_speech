import os
import types
import shutil
import datetime

path = os.path.join

try: shutil.rmtree(compiled_discus_dir := path(compiled_dir := "compiled", "discussions"))
except: pass
os.mkdir(compiled_discus_dir)
try: shutil.rmtree(compiled_partic_dir := path(compiled_dir, "participants"))
except: pass
os.mkdir(compiled_partic_dir)

def attrs(text):
    listobj = pairs(text)
    n = types.SimpleNamespace()
    for k, v in listobj:
        setattr(n, k, v)
    return n

def pairs(text):
    items = []
    nestlvl = 0
    name = ""
    openindex = 0
    escaped = False
    for idx, c in enumerate(text):
        if escaped:
            name.append(c)
            escaped = False
            continue
        if c == "(":
            if nestlvl == 0:
                openindex = idx
            nestlvl += 1
        elif c == ")":
            nestlvl -= 1
            if nestlvl == 0:
                items.append((name.strip(), text[openindex+1:idx]))
                name = ""
        elif c == "\\":
            escaped = True
        elif nestlvl == 0:
            name += c
    return items

def read(*path_parts):
    return open(path(*path_parts), encoding="utf-8").read()

def write(text, *path_parts):
    return open(path(*path_parts), "w", encoding="utf-8").write(text)

def templated(filepath, args):
    result = ""
    def part(arg, end="\n"):
        nonlocal result
        result += str(arg) + end
    locals_ = {"_": part, "attrs": attrs, "pairs": pairs, **args}
    exec(read(filepath), locals_, locals_)
    return result

def write_templated(inpath, outpath, args):
    result = templated(inpath, args)
    write(result, outpath)

def get_participant(filename):
    return attrs(read("participants", filename))

for participant_fname in os.listdir(participants_dir := "participants"):
    participant = attrs(read(participants_dir, participant_fname))
    participant.filename = participant_fname
    write_templated("participant.html.template", path(compiled_partic_dir, participant_fname), {"participant": participant})

discussions = []
for discussion_fname in os.listdir(discussions_dir := "discussions"):
    discussion = attrs(read(discussions_dir, discussion_fname))
    participants = set()
    discussion.filename = discussion_fname
    for sender_name, _ in pairs(discussion.messages):
        participants.add(sender_name)
    discussion.participants = participants
    discussions.append(discussion)
    write_templated("discussion.html.template", path(compiled_discus_dir, discussion_fname), {"discussion": discussion, "get_participant": get_participant})
discussions.sort(key=lambda discussion: datetime.datetime.strptime(discussion.date, "%d.%m.%Y %H:%M:%S"), reverse=True)
write_templated("index.html.template", path(compiled_dir, "index.html"), {"discussions": discussions, "get_participant": get_participant})
