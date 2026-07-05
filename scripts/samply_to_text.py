#!/usr/bin/env python3
"""Convert a samply/Firefox-Profiler .json.gz profile into a plain-text,
LLM-ingestable hot-path summary: a folded-stack file (func;func;func count)
plus a top-N self-time function list. See docs/profiling.md.

Function names in a saved samply profile are unresolved (raw hex addresses)
by default -- samply normally resolves them lazily when the profile is
viewed in-browser via `samply load`. To get real names in this offline
conversion, record with `--unstable-presymbolicate`, which writes a
`<profile>.syms.json` sidecar next to the `.json.gz` file; this script uses
it when present and falls back to raw addresses otherwise.
"""
import bisect
import gzip
import json
import os
import sys
from collections import Counter


def load_json_gz(path):
    with gzip.open(path, "rt", encoding="utf-8") as f:
        return json.load(f)


def sidecar_path(profile_path):
    base = profile_path[:-3] if profile_path.endswith(".gz") else profile_path
    return base + ".syms.json"


def build_symbolicator(profile, syms_path):
    """Returns lib_index -> (known_addresses dict, sorted symbol ranges) for
    every lib in `profile['libs']` that has a matching entry (by codeId) in
    the presymbolicate sidecar."""
    if not os.path.exists(syms_path):
        return None

    with open(syms_path, encoding="utf-8") as f:
        syms = json.load(f)
    string_table = syms["string_table"]

    def name_at(idx):
        return string_table[idx] if 0 <= idx < len(string_table) else "0x?"

    by_code_id = {entry["code_id"]: entry for entry in syms["data"]}

    resolvers = {}
    for lib_index, lib in enumerate(profile["libs"]):
        entry = by_code_id.get(lib.get("codeId"))
        if entry is None:
            continue
        # samply's --unstable-presymbolicate sidecar format is explicitly
        # unstable ("will probably change") — tolerate out-of-range string
        # indices rather than crashing the whole conversion over one bad entry.
        known = {addr: name_at(idx) for addr, idx in entry.get("known_addresses", [])}
        ranges = sorted(
            (row["rva"], row["rva"] + row.get("size", 0), name_at(row["symbol"]))
            for row in entry.get("symbol_table", [])
        )
        range_starts = [r[0] for r in ranges]
        resolvers[lib_index] = (known, ranges, range_starts)
    return resolvers


def resolve_frame_name(thread, frame_index, resolvers, libs):
    frame_table = thread["frameTable"]
    func_table = thread["funcTable"]
    resource_table = thread["resourceTable"]
    string_array = thread["stringArray"]

    func_index = frame_table["func"][frame_index]
    address = frame_table["address"][frame_index]
    fallback_name = string_array[func_table["name"][func_index]]

    if resolvers is None or address is None or address < 0:
        return fallback_name

    resource_index = func_table["resource"][func_index]
    if resource_index is None or resource_index < 0:
        return fallback_name
    lib_index = resource_table["lib"][resource_index]
    if lib_index is None:
        return fallback_name

    resolved = resolvers.get(lib_index)
    if resolved is None:
        return fallback_name
    known, ranges, range_starts = resolved

    if address in known:
        return known[address]

    pos = bisect.bisect_right(range_starts, address) - 1
    if 0 <= pos < len(ranges):
        start, end, name = ranges[pos]
        if start <= address < end:
            return name

    return fallback_name


def stack_frames(thread, stack_index, resolvers, libs, cache):
    if stack_index is None:
        return []
    if stack_index in cache:
        return cache[stack_index]
    stack_table = thread["stackTable"]
    prefix = stack_table["prefix"][stack_index]
    frame = stack_table["frame"][stack_index]
    name = resolve_frame_name(thread, frame, resolvers, libs)
    frames = stack_frames(thread, prefix, resolvers, libs, cache) + [name]
    cache[stack_index] = frames
    return frames


def collapse_thread(thread, resolvers, libs):
    folded = Counter()
    self_time = Counter()
    cache = {}
    for stack_index in thread.get("samples", {}).get("stack", []):
        if stack_index is None:
            continue
        frames = stack_frames(thread, stack_index, resolvers, libs, cache)
        if not frames:
            continue
        folded[";".join(frames)] += 1
        self_time[frames[-1]] += 1
    return folded, self_time


def main():
    if len(sys.argv) != 4:
        print(
            f"usage: {sys.argv[0]} <profile.json.gz> <folded-out.txt> <top-functions-out.txt>",
            file=sys.stderr,
        )
        sys.exit(1)

    profile_path, folded_path, top_path = sys.argv[1:4]
    profile = load_json_gz(profile_path)
    resolvers = build_symbolicator(profile, sidecar_path(profile_path))
    if resolvers is None:
        print(
            "warning: no *.syms.json sidecar found next to the profile — "
            "function names will be raw addresses. Record with "
            "`samply record --unstable-presymbolicate ...` for real names.",
            file=sys.stderr,
        )

    total_folded = Counter()
    total_self = Counter()
    for thread in profile.get("threads", []):
        folded, self_time = collapse_thread(thread, resolvers, profile["libs"])
        total_folded.update(folded)
        total_self.update(self_time)

    with open(folded_path, "w", encoding="utf-8") as f:
        for stack, count in sorted(total_folded.items(), key=lambda kv: -kv[1]):
            f.write(f"{stack} {count}\n")

    with open(top_path, "w", encoding="utf-8") as f:
        f.write("# Top functions by self-time (sample count)\n")
        for name, count in total_self.most_common(30):
            f.write(f"{count:8d}  {name}\n")

    print(f"wrote {len(total_folded)} folded stacks to {folded_path}")
    print(f"wrote top {min(30, len(total_self))} functions to {top_path}")


if __name__ == "__main__":
    main()
