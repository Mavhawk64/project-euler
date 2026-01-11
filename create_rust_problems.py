import csv
import os
import subprocess
from re import sub

with open(
    os.path.join(os.path.dirname(__file__), "pe_minimal_problems.csv"),
    "r",
    encoding="utf-8",
) as f:
    reader = csv.reader(f)
    next(reader)
    problems = [(int(row[0]), row[1]) for row in reader if row]


# https://labex.io/tutorials/convert-string-to-kebab-case-in-python-13675
def kebab(s):
    return "-".join(
        sub(
            r"(\s|_|-)+",
            " ",
            sub(
                r"[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+",
                lambda mo: " " + mo.group(0).lower(),
                s,
            ),
        ).split()
    )


def clean_name(s):
    # Remove invalid characters (keep only letters, numbers, spaces, underscores, and hyphens)
    s = sub(r"[^a-zA-Z0-9\s_-]+", "", s)
    # If it starts with a number, prefix with an underscore or a letter
    if s and s[0].isdigit():
        s = "p-" + s  # prefix with 'p' for "problem"
    return s


problems = [(i, kebab(clean_name(name))) for i, name in problems]

# [print(i, name) for i, name in problems]

for i, name in problems:
    START_RANGE = int((((i - 1) // 50) * 50 + 1))
    END_RANGE = int((START_RANGE + 49))
    RANGE_DIR = f"solutions_{START_RANGE}-{END_RANGE}"
    # print(RANGE_DIR)
    if os.path.isdir(
        os.path.join(os.path.dirname(__file__), f"{RANGE_DIR}", f"problem-{i}")
    ):
        # print(f"Problem {i} - {name} already has a directory!")
        continue
    subprocess.run(f"./create_new_rust_problem.sh {i} {name}", shell=True)
