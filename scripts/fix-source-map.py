#!/usr/bin/env python3
# Makes a wasm2map source map shippable: paths under the repository become
# relative, and the map embeds each source's contents so DevTools never has
# to fetch the file. Only the repository's own files are embedded;
# standard-library and dependency sources would bloat the map and get an
# empty string (wasm-opt's source-map parser rejects null).

import json
import os
import sys

repo_root = os.path.abspath(sys.argv[1])
map_path = sys.argv[2]

with open(map_path, encoding="utf-8") as file:
    source_map = json.load(file)

sources = []
contents = []
for source in source_map["sources"]:
    path = os.path.normpath(source)
    if os.path.isabs(path):
        relative = os.path.relpath(path, repo_root)
        label = path if relative.startswith("..") else relative
        embeddable = path.startswith(repo_root + os.sep)
    else:
        label = path
        embeddable = True
    sources.append(label)
    content = ""
    if embeddable:
        try:
            with open(path, encoding="utf-8") as file:
                content = file.read()
        except OSError:
            content = ""
    contents.append(content)

source_map["sources"] = sources
source_map["sourcesContent"] = contents

with open(map_path, "w", encoding="utf-8") as file:
    json.dump(source_map, file)
