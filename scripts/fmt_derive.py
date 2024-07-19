import os
import fileinput
import re
from typing import List, Dict

config: List[str] = [
    # bevy derives
    "Component",
    "Resource",
    "States",
    "SubStates",
    "Event",
    # rust derives
    "Copy",
    "Clone",
    "Default",
    "Debug",
    "PartialEq",
    "Eq",
    "PartialOrd",
    "Ord",
    "Hash",
    # serde derives
    "Serialize",
    "Deserialize",
]

pattern: re.Pattern = re.compile(
    r"^#\[derive\((\s*[a-zA-Z0-9_]+\s*,)*(\s*[a-zA-Z0-9_]+\s*)\)]$"
)
ordering: Dict[str, int] = {s: i for i, s in enumerate(config)}

for root, dirs, files in os.walk("."):
    if "target" in root:
        continue
    for name in files:
        path = root + os.sep + name

        if name.endswith(".rs"):
            print(path)
            for line in fileinput.input(path, inplace=True):
                if pattern.match(line):
                    derives = line[9:-3]
                    derives = [
                        (ordering.get(x.strip(), 100), x.strip())
                        for x in derives.split(",")
                    ]
                    derives.sort()
                    line = "#[derive(" + ", ".join(x[1] for x in derives) + ")]"
                    print(line)
                else:
                    print(line, end="")
