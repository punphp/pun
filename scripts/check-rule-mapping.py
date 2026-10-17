#!/usr/bin/env python3
"""Check crates/pun-config/rules/mapping.toml against the naming rules in docs/RULES.md.

Fails (exit 1) on: a `tentative` entry, a non-kebab-case or duplicate name (rule names and aliases share one namespace
across all commands), a lint name without a verb prefix, a broken `pun` / `fixed_by` / `conflicts` / `merged`
reference, an upstream entry that is not exactly one of pun / drop / unsupported, a Rector entry without its class,
a rule that no upstream entry maps to (unless it is marked `pun_only = true`), and a PHPStan/Larastan entry
without `variant` when its rule replaces several identifiers.

Usage: scripts/check-rule-mapping.py [path/to/mapping.toml]
"""
import re
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

DEFAULT = Path(__file__).resolve().parent.parent / "crates/pun-config/rules/mapping.toml"
KEBAB = re.compile(r"[a-z0-9]+(-[a-z0-9]+)*")
REF = re.compile(r"(format|lint|analyse)\.[a-z0-9-]+")
LINT_PREFIXES = ("no-", "prefer-", "require-", "add-")
# The refactor group (configurable codemods such as Rector RenameMethod) is named after the edit it makes.
REFACTOR_PREFIXES = ("rename-", "replace-", "remove-", "change-", "wrap-")
PHPSTAN_FAMILY = ("phpstan", "phpstan-strict-rules", "phpstan-deprecation-rules", "larastan")
# [analyse] switches (PHPStan levels 7-10) are settings, not rules.
SWITCH = re.compile(r"\[analyse\] [a-z-]+ = true")


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

    rules = data["rules"]
    ids = {f"{c}.{n}" for c, rs in rules.items() for n in rs}
    owner = {}
    for c, rs in rules.items():
        for n, r in rs.items():
            for name in [n, *r.get("aliases", [])]:
                if not KEBAB.fullmatch(name):
                    err(f"{c}.{n}: {name!r} is not kebab-case")
                if name in owner and owner[name] != f"{c}.{n}":
                    err(f"{c}.{n}: name {name!r} already used by {owner[name]}")
                owner[name] = f"{c}.{n}"
            if r.get("status") != "decided":
                err(f"{c}.{n}: status is {r.get('status')!r}")

    for n, r in rules.get("lint", {}).items():
        ok = n.startswith(LINT_PREFIXES)
        ok = ok or (r.get("group") == "refactor" and n.startswith(REFACTOR_PREFIXES))
        if not ok:
            err(f"lint.{n}: lint names start with {'/'.join(LINT_PREFIXES)}")
    for n in rules.get("analyse", {}):
        if n.startswith(("prefer-", "require-", "add-")):
            err(f"analyse.{n}: analyse names describe the problem, not a fix")

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

    for c, rs in rules.items():
        for n, r in rs.items():
            rid = f"{c}.{n}"
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
                if f.startswith("analyse.") or f not in ids:
                    err(f"{rid}: fixed_by {f!r} is not a lint or format rule")
            for text in (r.get("note", ""), r.get("summary", "")):
                for ref in REF.finditer(text):
                    if ref.group(0) not in ids:
                        err(f"{rid}: text mentions unknown rule {ref.group(0)!r}")

    for pair in data["meta"].get("conflicts", []):
        for x in pair:
            if x not in ids:
                err(f"meta.conflicts: {x!r} is not a rule")
    return errors


def selftest():
    good = {
        "meta": {"conflicts": []},
        "rules": {"lint": {"no-x": {"status": "decided", "aliases": ["old-x"]}},
                  "analyse": {"undefined-x": {"status": "decided", "fixed_by": ["lint.no-x"]}}},
        "upstream": {"mago-linter": {"x": {"pun": "lint.no-x", "status": "decided"}},
                     "phpstan": {"x.notFound": {"pun": "analyse.undefined-x", "status": "decided"}}},
    }
    assert check(good) == [], check(good)
    import copy

    def bad(mutate, expect):
        d = copy.deepcopy(good)
        mutate(d)
        out = check(d)
        assert any(expect in e for e in out), (expect, out)

    bad(lambda d: d["rules"]["lint"]["no-x"].update(status="tentative"), "tentative")
    bad(lambda d: d["rules"]["lint"].update({"x-thing": {"status": "decided", "pun_only": True}}), "lint names start")
    bad(lambda d: d["rules"]["analyse"]["undefined-x"].update(aliases=["no-x"]), "already used")
    bad(lambda d: d["rules"]["analyse"]["undefined-x"].update(aliases=["Bad_Name"]), "kebab-case")
    bad(lambda d: d["upstream"]["mago-linter"]["x"].update(pun="lint.gone"), "is not a rule")
    bad(lambda d: d["rules"]["lint"].update({"no-y": {"status": "decided"}}), "no upstream entry")
    bad(lambda d: d["rules"]["analyse"]["undefined-x"].update(fixed_by=["analyse.undefined-x"]), "fixed_by")
    bad(lambda d: d["upstream"].update(rector={"Foo": {"pun": "lint.no-x", "status": "decided"}}), "class")
    bad(lambda d: d["rules"]["lint"]["no-x"].update(merged=["mago-linter:x", "rector:Nope"]), "merged source")
    bad(lambda d: d["meta"].update(conflicts=[["lint.no-x", "lint.nope"]]), "meta.conflicts")
    bad(lambda d: d["upstream"]["phpstan"].update({"y.notFound": {"pun": "analyse.undefined-x", "status": "decided"}}),
        "needs variant")
    bad(lambda d: d["upstream"]["phpstan"]["x.notFound"].update(variant="other.id"), "variant must be")


if __name__ == "__main__":
    selftest()
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT
    data = tomllib.loads(path.read_text())
    errors = check(data)
    for e in errors:
        print(e)
    counts = {c: len(rs) for c, rs in data["rules"].items()}
    print(f"{path.name}: {counts}, {sum(len(u) for u in data['upstream'].values())} upstream entries, "
          f"{len(errors)} error(s)")
    sys.exit(1 if errors else 0)
