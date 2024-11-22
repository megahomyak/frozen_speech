import os
import json

for file_name in os.listdir(discussions := "discussions"):
    discussion = json.read(open(f"{discussions}/{file_path}"))
    os.makedirs("compiled/discussions", exists_ok=True)
    index = ["<!DOCTYPE html>"]
    def tag(param):
        if isinstance(param, type(tag)):
            tag(f"<{function.__name__}>")
            param()
            tag(f"</{function.__name__}>")
        else:
            index.append(str(param))
    @tag
    def html():
        @tag
        def head():
            tag("")
        @tag
        def body():
            pass
    open("compiled/index.html", "w").write("".join(index))
