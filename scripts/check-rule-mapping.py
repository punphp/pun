#!/usr/bin/env python3
"""Check crates/pun-config/rules/mapping.toml against context/plan/rules.md §3/§5 and config.md §4-§5.

Fails (exit 1) on: a `tentative` entry; a rule key that isn't "<group>/<name>"; a group not in [meta.groups]; a name
that isn't kebab-case, is listed twice in its group, or repeats its group; a lint name without its verb prefix (codemod
prefixes only in the refactor group); an analyse name phrased as a fix; a field from before the rename (aliases,
tier, severity, group); format `presets`, lint `sets` or analyse `preset` outside their lists; a broken `pun` /
`fixed_by` / `conflicts` / `merged` reference; an upstream entry that is not exactly one of pun / drop / unsupported; a
Rector entry without its class; a rule that no upstream entry maps to (unless `pun_only = true`); a PHPStan/Larastan
entry without `variant` when its rule replaces several identifiers; and a conflict whose two sides one format preset,
the lint sets together, or one analyse preset would enable. Prints rule counts per command, set and preset.

Usage: scripts/check-rule-mapping.py [path/to/mapping.toml]
"""
import re
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

DEFAULT = Path(__file__).resolve().parent.parent / "crates/pun-config/rules/mapping.toml"
COMMANDS = ("format", "lint", "analyse")
KEBAB = re.compile(r"[a-z0-9]+(-[a-z0-9]+)*")
# Topic and ecosystem groups are kebab-case; upgrade groups name their target (php-8.5, laravel-13, laravel-5.8).
GROUP = re.compile(r"[a-z0-9]+(-[a-z0-9]+)*|(php|laravel|phpunit|pest)-\d+(\.\d+)?")
REF = re.compile(r"\b(?:format|lint|analyse)/[a-z0-9.-]+/[a-z0-9-]+")
LINT_PREFIXES = ("no-", "prefer-", "require-", "add-")
# Configurable codemods (Rector RenameMethod, CS-Fixer general_attribute_remove) are named after their edit.
CODEMOD_PREFIXES = ("rename-", "replace-", "remove-", "change-", "wrap-")
# Decided names that read the group word as part of a term (format.md §8).
REPEATS_GROUP = {"format/whitespace/no-blank-line-whitespace"}
GONE = ("aliases", "tier", "severity", "group")
PHPSTAN_FAMILY = ("phpstan", "phpstan-strict-rules", "phpstan-deprecation-rules", "larastan")
# [analyse] switches (PHPStan levels 7-10) are settings, not rules.
SWITCH = re.compile(r"\[analyse\] [a-z-]+ = true")


def duplicates(text):
    """Rule keys listed twice in one [rules.<command>] section (TOML refuses the file; this names them)."""
    out, section, seen = [], None, set()
    for line in text.splitlines():
        if line.startswith("["):
            section, seen = line, set()
            continue
        if section and section.startswith("[rules.") and " = " in line:
            k = line.split(" = ", 1)[0].strip('"')
            if k in seen:
                out.append(f"{section[1:-1]}: {k!r} is listed twice")
            seen.add(k)
    return out


def check(data):
    errors = []
    err = errors.append

    def walk(node, path):
        if isinstance(node, dict):
            for k, v in node.items():
                walk(v, f"{path}.{k}")
        elif isinstance(node, list):
            for v in node:
                walk(v, path)
        elif node == "tentative":
            err(f"{path}: still tentative")

    walk(data, "")

    meta = data["meta"]
    groups = meta.get("groups", {})
    for g, desc in groups.items():
        if not GROUP.fullmatch(g):
            err(f"meta.groups: {g!r} is not a group name")
        if not isinstance(desc, str) or not desc:
            err(f"meta.groups: {g!r} needs a one-line description")
    sets = meta.get("sets", [])
    presets = meta.get("presets", {})
    rules = data["rules"]
    ids = set()
    for c in COMMANDS:
        for key, r in rules.get(c, {}).items():
            rid = f"{c}/{key}"
            ids.add(rid)
            g, _, n = key.partition("/")
            if not n or "/" in n:
                err(f"{rid}: the key must be <group>/<name>")
                continue
            if g not in groups:
                err(f"{rid}: group {g!r} is not in [meta.groups]")
            if not KEBAB.fullmatch(n):
                err(f"{rid}: {n!r} is not kebab-case")
            if f"-{g}-" in f"-{n}-" and rid not in REPEATS_GROUP:
                err(f"{rid}: the name repeats its group")
            if r.get("status") != "decided":
                err(f"{rid}: status is {r.get('status')!r}")
            for f in GONE:
                if f in r:
                    err(f"{rid}: `{f}` is gone (the group is in the key; no aliases, tiers or severities)")
            if c == "lint":
                ok = n.startswith(LINT_PREFIXES) or (g == "refactor" and n.startswith(CODEMOD_PREFIXES))
                if not ok:
                    err(f"{rid}: lint names start with {'/'.join(LINT_PREFIXES)} (codemods: refactor/rename- ...)")
            if c == "analyse" and n.startswith(("prefer-", "require-", "add-")):
                err(f"{rid}: analyse names describe the problem, not a fix")
            for field, cmd in (("presets", "format"), ("sets", "lint"), ("preset", "analyse")):
                if field in r and c != cmd:
                    err(f"{rid}: `{field}` is for {cmd} rules only")
            for p in r.get("presets", []) if c == "format" else []:
                if p not in presets.get("format", []) or p == "empty":
                    err(f"{rid}: presets entry {p!r} is not a format preset")
            for s in r.get("sets", []) if c == "lint" else []:
                if s not in sets:
                    err(f"{rid}: set {s!r} is not in meta.sets")
            if c == "analyse" and "preset" in r and r["preset"] not in presets.get("analyse", []):
                err(f"{rid}: preset {r['preset']!r} is not an analyse preset")

    sources = defaultdict(set)
    for tool, entries in data["upstream"].items():
        for key, u in entries.items():
            where = f"upstream.{tool}.{key}"
            kinds = [k for k in ("pun", "drop", "unsupported") if k in u]
            if len(kinds) != 1:
                err(f"{where}: needs exactly one of pun / drop / unsupported, has {kinds}")
            if "pun" in u:
                if SWITCH.fullmatch(u["pun"]):
                    continue
                if u["pun"] not in ids:
                    err(f"{where}: pun = {u['pun']!r} is not a rule")
                sources[u["pun"]].add(f"{tool}:{key}")
            if tool.startswith("rector") or tool == "pest-plugin-rector":
                if "\\" not in key and "\\" not in u.get("class", ""):
                    err(f"{where}: Rector entry without its fully qualified `class`")

    family = defaultdict(int)
    for tool in PHPSTAN_FAMILY:
        for u in data["upstream"].get(tool, {}).values():
            if "pun" in u:
                family[u["pun"]] += 1
    for tool, entries in data["upstream"].items():
        for key, u in entries.items():
            where = f"upstream.{tool}.{key}"
            if "variant" in u and (tool not in PHPSTAN_FAMILY or u["variant"] != key):
                err(f"{where}: variant must be the PHPStan identifier itself, on phpstan/larastan entries only")
            if tool in PHPSTAN_FAMILY and family.get(u.get("pun"), 0) > 1 and "variant" not in u:
                err(f"{where}: {u['pun']} replaces several identifiers, so this entry needs variant = {key!r}")

    for c in COMMANDS:
        for key, r in rules.get(c, {}).items():
            rid = f"{c}/{key}"
            if not sources[rid] and not r.get("pun_only"):
                err(f"{rid}: no upstream entry maps to it (mark Pun-only rules with pun_only = true)")
            if sources[rid] and r.get("pun_only"):
                err(f"{rid}: pun_only but {sorted(sources[rid])} map to it")
            for m in r.get("merged", []):
                if m not in sources[rid]:
                    err(f"{rid}: merged source {m!r} does not map to this rule")
            if r.get("merged") and len(r["merged"]) < 2:
                err(f"{rid}: merged lists fewer than two sources")
            for f in r.get("fixed_by", []):
                if not f.startswith(("lint/", "format/")) or f not in ids:
                    err(f"{rid}: fixed_by {f!r} is not a lint or format rule")
            for text in (r.get("note", ""), r.get("summary", "")):
                for ref in REF.finditer(text):
                    if ref.group(0) not in ids:
                        err(f"{rid}: text mentions unknown rule {ref.group(0)!r}")

    for pair in meta.get("conflicts", []):
        for x in pair:
            if x not in ids:
                err(f"meta.conflicts: {x!r} is not a rule")
    for name, on in enablers(data).items():
        for a, b in meta.get("conflicts", []):
            if a in on and b in on:
                err(f"{name} enables both sides of conflict {a} / {b}")
    return errors


def enabled(data, cmd, test):
    return {f"{cmd}/{k}" for k, r in data["rules"].get(cmd, {}).items() if test(r)}


def analyse_preset(data, preset):
    """The analyse rules `[analyse] preset = <preset>` enables (each preset includes the ones below it)."""
    order = data["meta"]["presets"]["analyse"]
    k = order.index(preset)
    return enabled(data, "analyse", lambda r: r.get("preset") in order[: k + 1])


def enablers(data):
    """Every rule set a config can turn on at once: each format preset, all lint sets together, each analyse preset."""
    out = {}
    for p in data["meta"]["presets"]["format"]:
        out[f"format preset {p}"] = enabled(data, "format", lambda r: p in r.get("presets", []))
    out["the lint sets together"] = enabled(data, "lint", lambda r: bool(r.get("sets")))
    for p in data["meta"]["presets"]["analyse"]:
        out[f"analyse preset {p}"] = analyse_preset(data, p)
    return out


def selftest():
    good = {
        "meta": {"conflicts": [], "groups": {"dead-code": "Dead code.", "undefined": "Undefined.",
                                             "refactor": "Codemods.", "php-8.4": "Upgrade."},
                 "sets": ["dead-code"], "presets": {"format": ["laravel", "empty"], "analyse": ["basic", "max"]}},
        "rules": {"lint": {"dead-code/no-x": {"status": "decided", "sets": ["dead-code"]}},
                  "analyse": {"undefined/x": {"status": "decided", "preset": "basic",
                                              "fixed_by": ["lint/dead-code/no-x"]}}},
        "upstream": {"mago-linter": {"x": {"pun": "lint/dead-code/no-x", "status": "decided"}},
                     "phpstan": {"x.notFound": {"pun": "analyse/undefined/x", "status": "decided"}}},
    }
    assert check(good) == [], check(good)
    import copy

    def add(cmd, key, **r):
        return lambda d: d["rules"].setdefault(cmd, {}).update({key: {"status": "decided", "pun_only": True, **r}})

    def bad(mutate, expect):
        d = copy.deepcopy(good)
        mutate(d)
        out = check(d)
        assert any(expect in e for e in out), (expect, out)

    def fine(mutate):
        d = copy.deepcopy(good)
        mutate(d)
        assert check(d) == [], check(d)

    fine(add("lint", "refactor/rename-x"))
    fine(add("lint", "php-8.4/prefer-x", sets=["dead-code"]))
    bad(lambda d: d["rules"]["lint"]["dead-code/no-x"].update(status="tentative"), "tentative")
    bad(add("lint", "dead-code/x-thing"), "lint names start")
    bad(add("lint", "dead-code/rename-x"), "lint names start")
    bad(add("analyse", "undefined/prefer-x"), "describe the problem")
    bad(add("lint", "unknown/no-y"), "not in [meta.groups]")
    bad(add("lint", "no-y"), "<group>/<name>")
    bad(add("lint", "dead-code/no-Bad"), "kebab-case")
    bad(add("lint", "dead-code/no-dead-code-y"), "repeats its group")
    bad(add("analyse", "undefined/undefined"), "repeats its group")
    bad(add("lint", "dead-code/no-y", aliases=["old"]), "`aliases` is gone")
    bad(add("lint", "dead-code/no-y", tier="strict"), "`tier` is gone")
    bad(add("lint", "dead-code/no-y", sets=["typo"]), "not in meta.sets")
    bad(add("lint", "dead-code/no-y", preset="basic"), "for analyse rules only")
    bad(add("analyse", "undefined/y", preset="pedantic"), "not an analyse preset")
    bad(add("format", "dead-code/y", presets=["empty"]), "not a format preset")
    bad(lambda d: d["meta"]["groups"].update({"Bad Group": "x"}), "not a group name")
    bad(lambda d: d["meta"]["groups"].update({"x": ""}), "one-line description")
    bad(lambda d: d["upstream"]["mago-linter"]["x"].update(pun="lint/dead-code/gone"), "is not a rule")
    bad(lambda d: d["rules"]["lint"].update({"dead-code/no-y": {"status": "decided"}}), "no upstream entry")
    bad(lambda d: d["rules"]["analyse"]["undefined/x"].update(fixed_by=["analyse/undefined/x"]), "fixed_by")
    bad(lambda d: d["upstream"].update(rector={"Foo": {"pun": "lint/dead-code/no-x", "status": "decided"}}), "class")
    bad(lambda d: d["rules"]["lint"]["dead-code/no-x"].update(merged=["mago-linter:x", "rector:Nope"]),
        "merged source")
    bad(lambda d: d["rules"]["lint"]["dead-code/no-x"].update(summary="see lint/dead-code/nope"), "unknown rule")
    bad(lambda d: d["meta"].update(conflicts=[["lint/dead-code/no-x", "lint/dead-code/nope"]]), "meta.conflicts")
    bad(lambda d: d["upstream"]["phpstan"].update({"y.notFound": {"pun": "analyse/undefined/x", "status": "decided"}}),
        "needs variant")
    bad(lambda d: d["upstream"]["phpstan"]["x.notFound"].update(variant="other.id"), "variant must be")

    def conflict(d):
        add("lint", "dead-code/no-y", sets=["dead-code"])(d)
        d["meta"]["conflicts"] = [["lint/dead-code/no-x", "lint/dead-code/no-y"]]
    bad(conflict, "the lint sets together enables both sides")

    def analyse_conflict(d):
        add("analyse", "undefined/y", preset="max")(d)
        d["meta"]["conflicts"] = [["analyse/undefined/x", "analyse/undefined/y"]]
    bad(analyse_conflict, "analyse preset max enables both sides")

    def format_conflict(d):
        add("format", "dead-code/a", presets=["laravel"])(d)
        add("format", "dead-code/b", presets=["laravel"])(d)
        d["meta"]["conflicts"] = [["format/dead-code/a", "format/dead-code/b"]]
    bad(format_conflict, "format preset laravel enables both sides")
    assert analyse_preset(good, "basic") == {"analyse/undefined/x"}
    d = copy.deepcopy(good)
    d["rules"]["analyse"]["undefined/x"]["preset"] = "max"
    assert analyse_preset(d, "basic") == set() and analyse_preset(d, "max") == {"analyse/undefined/x"}
    assert duplicates("[rules.lint]\n\"a/no-x\" = {}\n\"a/no-x\" = {}\n[rules.analyse]\n\"a/no-x\" = {}\n") == [
        "rules.lint: 'a/no-x' is listed twice"]


if __name__ == "__main__":
    selftest()
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT
    text = path.read_text()
    dups = duplicates(text)
    if dups:
        print("\n".join(dups))
        sys.exit(1)
    data = tomllib.loads(text)
    errors = check(data)
    for e in errors:
        print(e)
    for c in COMMANDS:
        per = defaultdict(int)
        for key in data["rules"][c]:
            per[key.split("/")[0]] += 1
        print(f"{c}: {len(data['rules'][c])} rules in {len(per)} groups")
    print("format presets: " + ", ".join(
        f"{p} {sum(p in r.get('presets', []) for r in data['rules']['format'].values())}"
        for p in data["meta"]["presets"]["format"]))
    print("lint sets: " + ", ".join(
        f"{s} {sum(s in r.get('sets', []) for r in data['rules']['lint'].values())}" for s in data["meta"]["sets"])
        + f", no set {sum(not r.get('sets') for r in data['rules']['lint'].values())}")
    print("analyse presets (cumulative): " + ", ".join(
        f"{p} {len(analyse_preset(data, p))}" for p in data["meta"]["presets"]["analyse"])
        + f", opt-in {sum('preset' not in r for r in data['rules']['analyse'].values())}")
    print(f"{path.name}: {sum(len(r) for r in data['rules'].values())} rules, "
          f"{sum(len(u) for u in data['upstream'].values())} upstream entries, {len(errors)} error(s)")
    sys.exit(1 if errors else 0)
