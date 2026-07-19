#!/usr/bin/env python3
"""Remove inline #[cfg(test)] modules from LCOV line totals and enforce a gate."""

from __future__ import annotations

import argparse
import pathlib
import re
import sys


TEST_MODULE = re.compile(
    r"(?m)^[ \t]*#\[cfg\(test\)\][ \t]*\n[ \t]*mod[ \t]+[A-Za-z_]\w*[ \t]*\{"
)


def matching_brace(text: str, opening: int) -> int:
    depth = 1
    index = opening + 1
    while index < len(text):
        if text.startswith("//", index):
            newline = text.find("\n", index + 2)
            index = len(text) if newline < 0 else newline + 1
            continue
        if text.startswith("/*", index):
            comment_depth = 1
            index += 2
            while index < len(text) and comment_depth:
                if text.startswith("/*", index):
                    comment_depth += 1
                    index += 2
                elif text.startswith("*/", index):
                    comment_depth -= 1
                    index += 2
                else:
                    index += 1
            continue
        raw = re.match(r'(?:br|r)(?P<hashes>#+)?"', text[index:])
        if raw:
            terminator = '"' + (raw.group("hashes") or "")
            end = text.find(terminator, index + raw.end())
            if end < 0:
                raise ValueError("unterminated raw string while locating #[cfg(test)] module")
            index = end + len(terminator)
            continue
        if text[index] == '"':
            index += 1
            while index < len(text):
                if text[index] == "\\":
                    index += 2
                elif text[index] == '"':
                    index += 1
                    break
                else:
                    index += 1
            continue
        char = re.match(r"'(?:\\.|[^\\'\n])'", text[index:])
        if char:
            index += char.end()
            continue
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise ValueError("unterminated #[cfg(test)] module")


def test_ranges(source: pathlib.Path) -> list[tuple[int, int]]:
    try:
        text = source.read_text(encoding="utf-8")
    except OSError:
        return []
    ranges = []
    for match in TEST_MODULE.finditer(text):
        closing = matching_brace(text, match.end() - 1)
        start_line = text.count("\n", 0, match.start()) + 1
        end_line = text.count("\n", 0, closing) + 1
        ranges.append((start_line, end_line))
    return ranges


def filter_record(record: list[str]) -> tuple[list[str], int, int]:
    source_line = next((line for line in record if line.startswith("SF:")), None)
    ranges = test_ranges(pathlib.Path(source_line[3:])) if source_line else []
    kept: list[str] = []
    found = hit = 0
    for line in record:
        if line.startswith("DA:"):
            line_number, count, *_ = line[3:].split(",")
            if any(start <= int(line_number) <= end for start, end in ranges):
                continue
            found += 1
            hit += int(count) > 0
        elif line.startswith(("LF:", "LH:")):
            continue
        kept.append(line)
    kept.extend((f"LF:{found}", f"LH:{hit}"))
    kept.append("end_of_record")
    return kept, found, hit


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=pathlib.Path)
    parser.add_argument("output", type=pathlib.Path)
    parser.add_argument("--minimum", type=float, default=80.0)
    args = parser.parse_args()

    filtered: list[str] = []
    total = covered = 0
    record: list[str] = []
    for line in args.input.read_text(encoding="utf-8").splitlines():
        if line == "end_of_record":
            if record:
                result, found, hit = filter_record(record)
                filtered.extend(result)
                total += found
                covered += hit
                record = []
        else:
            record.append(line)
    if record:
        result, found, hit = filter_record(record)
        filtered.extend(result)
        total += found
        covered += hit
    args.output.write_text("\n".join(filtered) + "\n", encoding="utf-8")

    if total == 0:
        print("error: LCOV report contains no production lines", file=sys.stderr)
        return 1
    percent = covered * 100.0 / total
    print(f"Production line coverage: {covered}/{total} ({percent:.2f}%)")
    if percent + 1e-9 < args.minimum:
        print(f"error: production line coverage is below {args.minimum:.2f}%", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
