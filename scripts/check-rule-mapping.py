#!/usr/bin/env python3
"""Check crates/pun-config/rules/mapping.toml against context/plan/rules.md §3/§5 and config.md §4-§5.

Fails (exit 1) on: a `tentative` entry; a rule key that isn't "<group>/<name>"; a group not in [meta.groups]; a name
that isn't kebab-case, is listed twice in its group, or repeats its group; a lint name without its verb prefix (codemod
prefixes only in the refactor group); an analyse name phrased as a fix; a field from before the rename (aliases,
tier, severity, group); format `presets`, lint `sets` or analyse `preset` outside their lists; a broken `pun` /
`fixed_by` / `conflicts` / `merged` reference; an upstream entry that is not exactly one of pun / drop / unsupported; a
Rector entry without its class; a rule that no upstream entry maps to (unless `pun_only = true`); a PHPStan/Larastan
entry without `variant` when its rule replaces several identifiers; `also` (the further rules a Mago analyzer code
reports for) outside mago-analyzer entries with `pun`, or naming no other rule; and a conflict whose two sides one format preset,
the lint sets together, or one analyse preset would enable.

Against the pinned upstream inventories in crates/pun-config/rules/upstream/*.json (context/plan/rules.md §5) it also
fails on: an in-scope Rector, rector-laravel, rector-phpunit or Pest rule, or an in-scope PHPStan, strict-rules,
deprecation-rules or Larastan rule class, that has no entry or maps to no Pun rule; an entry that maps a deprecated or
out-of-scope upstream rule (old PHP or Laravel upgrade set, no set, outside the in-scope sets) to a Pun rule; a lint or
analyse rule with no or several primary sources (one Rector rule, CS-Fixer fixer, Pint fixer or PHPStan-family rule
class) unless it is Mago-derived (`origin = "mago"`, Mago sources only) or `pun_only`; `merged` on a lint or analyse
rule; a class entry whose identifiers differ from the inventory; an emitted identifier with no entry, or one whose rule
is not the rule of a class that emits it; and a rule with a Rector source in an upgrade group below the floor (PHP 8.4,
Laravel 12). Prints rule counts per command, set, preset and source.

Usage: scripts/check-rule-mapping.py [path/to/mapping.toml]
"""
import json
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
INVENTORY = DEFAULT.parent / "upstream"
RECTOR_TOOLS = ("rector", "rector-laravel", "pest-plugin-rector", "rector-phpunit", "rector-symfony")
# A lint or analyse rule has exactly one primary source from these sections (the one-to-one rule, PLAN.md §2).
PRIMARY = ("php-cs-fixer", "pint", "phpstan-classes") + RECTOR_TOOLS
# Mago-derived rules (`origin = "mago"`) have only these.
MAGO = ("pun", "mago-linter", "mago-analyzer")
# Plan-level exclusions on top of the inventories' own scope (rules.md §2 O9): rector-symfony, PHPStan's extension API.
EXCLUDED_PREFIXES = ("Rector\\Symfony\\", "PHPStan\\Rules\\Api\\")
# Upgrade groups below the floor (PHP 8.4, Laravel 12): no rule with a Rector source may sit in one.
BELOW_FLOOR = re.compile(r"php-([57]\.\d|8\.[0-3])|laravel-([5-9](\.\d)?|1[01])")


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
            # `later = true`: after 1.0 (PLAN.md §2); only a lint rule that is not ported yet can wait.
            if "later" in r and (c != "lint" or r["later"] is not True or r.get("ported")):
                err(f"{rid}: `later = true` is only for lint rules that are not ported")
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
            # Further rules a Mago analyzer code reports for (context/reports/analyse-spike.md): evidence, not sources.
            if "also" in u and (tool != "mago-analyzer" or "pun" not in u):
                err(f"{where}: `also` is for mago-analyzer entries with `pun` only")
            for a in u.get("also", []):
                if a not in ids or a == u.get("pun"):
                    err(f"{where}: also {a!r} is not another rule")
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
    # PHPStan parameters (scripts/gen-phpstan-parameters.py): a level parameter, or an option of analyse rules.
    for name, p in data.get("phpstan-parameters", {}).items():
        where = f"phpstan-parameters.{name}"
        if p.get("level") is True:
            if set(p) != {"level"}:
                err(f"{where}: a level parameter has nothing else")
        elif set(p) != {"option", "rules"} or not p["rules"]:
            err(f"{where}: needs `option` and a non-empty `rules`, or `level = true`")
        else:
            for r in p["rules"]:
                if not r.startswith("analyse/") or r not in ids:
                    err(f"{where}: {r!r} is not an analyse rule")
    return errors


def inventory(directory=INVENTORY):
    """The pinned upstream inventories: Rector-family rules by class (a core rule a Laravel set configures is listed in
    both files and is in scope when either says so) and PHPStan-family rule classes by class."""
    def load(name):
        return json.loads((directory / f"{name}.json").read_text())
    rector = {}
    for r in load("rector-core") + load("rector-ecosystem"):
        o = rector.setdefault(r["fqcn"], dict(r))
        if r["in_scope"]:
            o["in_scope"] = True
    phpstan = {r["fqcn"]: r for r in load("phpstan-core") + load("phpstan-extensions")}
    return rector, phpstan


def in_scope(r):
    return r["in_scope"] and not r["deprecated"] and not r["fqcn"].startswith(EXCLUDED_PREFIXES)


def why_out(r):
    if r["deprecated"]:
        return "deprecated upstream"
    return r.get("out_of_scope_reason") or "out of scope (rules.md §2 O9)"


def check_upstream(data, inv):
    """The one-to-one rule against the inventories (PLAN.md §2, 2026-10-17)."""
    rector, phpstan = inv
    errors = []
    err = errors.append
    up = data["upstream"]

    seen = defaultdict(list)
    for tool in RECTOR_TOOLS:
        for key, u in up.get(tool, {}).items():
            cls = u.get("class", key)
            seen[cls].append(f"upstream.{tool}.{key}")
            r = rector.get(cls)
            where = f"upstream.{tool}.{key}"
            if "pun" in u and r is None:
                err(f"{where}: {cls} is not a rule in the pinned inventory, so it maps to no Pun rule")
            elif "pun" in u and not in_scope(r):
                err(f"{where}: {why_out(r)}, so it maps to no Pun rule")
            elif r is not None and in_scope(r) and "pun" not in u:
                err(f"{where}: in scope upstream but maps to no Pun rule")
    for cls, r in rector.items():
        if in_scope(r) and not seen[cls]:
            err(f"{cls}: in scope upstream but has no [upstream.*] entry")
        if len(seen[cls]) > 1:
            err(f"{cls}: listed {len(seen[cls])} times ({', '.join(seen[cls])})")

    classes = up.get("phpstan-classes", {})
    by_class = defaultdict(list)
    for key, u in classes.items():
        cls = u.get("class", "")
        by_class[cls].append(key)
        r = phpstan.get(cls)
        where = f"upstream.phpstan-classes.{key}"
        if r is None:
            err(f"{where}: {cls!r} is not a rule class in the pinned inventory")
            continue
        if "pun" in u and not in_scope(r):
            err(f"{where}: {why_out(r)}, so it maps to no Pun rule")
        if "pun" not in u and in_scope(r):
            err(f"{where}: in scope upstream but maps to no Pun rule")
        if u.get("identifiers", []) != r["identifiers"]:
            err(f"{where}: identifiers differ from the inventory")
    for cls in phpstan:
        if len(by_class[cls]) != 1:
            err(f"{cls}: needs exactly one [upstream.phpstan-classes] entry, has {len(by_class[cls])}")

    # One primary source per lint and analyse rule.
    primary, mago = defaultdict(list), defaultdict(list)
    for tool, entries in up.items():
        for key, u in entries.items():
            p = u.get("pun")
            if p and tool in PRIMARY:
                primary[p].append(f"{tool}:{key}")
            elif p and tool in MAGO:
                mago[p].append(f"{tool}:{key}")
    for c in ("lint", "analyse"):
        for key, r in data["rules"].get(c, {}).items():
            rid = f"{c}/{key}"
            if "merged" in r:
                err(f"{rid}: `merged` is gone (one upstream rule per Pun rule)")
            if r.get("origin") == "mago":
                if primary[rid] or not mago[rid]:
                    err(f"{rid}: origin = \"mago\" needs Mago sources and no primary source, has {primary[rid]}")
            elif not r.get("pun_only") and len(primary[rid]) != 1:
                err(f"{rid}: needs exactly one primary upstream rule, has {sorted(primary[rid]) or 'none'}")
            g = key.split("/")[0]
            if BELOW_FLOOR.fullmatch(g) and any(s.split(":")[0] in RECTOR_TOOLS for s in primary[rid]):
                err(f"{rid}: group {g} is an upgrade below the floor (PHP 8.4, Laravel 12)")

    # Every emitted identifier has an entry, and it points at the rule of a class that emits it.
    rule_of = {u.get("class"): u["pun"] for u in classes.values() if "pun" in u}
    emitters = defaultdict(set)
    for cls, r in phpstan.items():
        if cls in rule_of:
            for i in r["identifiers"]:
                emitters[i].add(rule_of[cls])
    family = {}
    for tool in PHPSTAN_FAMILY:
        family.update(up.get(tool, {}))
    patterns = [re.compile(re.escape(k).replace("%s", ".*")) for k in family if "%s" in k]
    for i, rules in sorted(emitters.items()):
        if "<" in i or i.startswith("phpstanApi."):
            continue
        if i not in family and not any(p.fullmatch(i) for p in patterns):
            err(f"identifier {i}: emitted by {sorted(rules)[0]} but has no [upstream.phpstan*] entry")
        elif "pun" in family.get(i, {}):
            p = family[i]["pun"]
            if p not in rules and not p.startswith("format/"):
                err(f"identifier {i}: maps to {p}, which no class that emits it maps to")
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
    bad(lambda d: d["rules"]["lint"]["dead-code/no-x"].update(later=True, ported=True), "only for lint rules that are not ported")
    bad(lambda d: d["rules"]["analyse"]["undefined/x"].update(later=True), "only for lint rules that are not ported")
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
    bad(lambda d: d["upstream"]["phpstan"]["x.notFound"].update(also=["lint/dead-code/no-x"]), "`also` is for")
    bad(lambda d: d["upstream"].update(**{"mago-analyzer": {"X": {"pun": "analyse/undefined/x", "also": ["analyse/undefined/x"]}}}),
        "is not another rule")
    fine(lambda d: d["upstream"].update(**{"mago-analyzer": {"X": {"pun": "analyse/undefined/x", "also": ["lint/dead-code/no-x"]}}}))

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
    bad(lambda d: d.update(**{"phpstan-parameters": {"p": {"option": "p", "rules": ["analyse/undefined/gone"]}}}),
        "is not an analyse rule")
    bad(lambda d: d.update(**{"phpstan-parameters": {"p": {"level": True, "option": "p"}}}), "has nothing else")
    bad(lambda d: d.update(**{"phpstan-parameters": {"p": {"option": "p", "rules": []}}}), "needs `option`")
    fine(lambda d: d.update(**{"phpstan-parameters": {"p": {"option": "p", "rules": ["analyse/undefined/x"]},
                                                      "q": {"level": True}}}))
    assert analyse_preset(good, "basic") == {"analyse/undefined/x"}
    d = copy.deepcopy(good)
    d["rules"]["analyse"]["undefined/x"]["preset"] = "max"
    assert analyse_preset(d, "basic") == set() and analyse_preset(d, "max") == {"analyse/undefined/x"}
    assert duplicates("[rules.lint]\n\"a/no-x\" = {}\n\"a/no-x\" = {}\n[rules.analyse]\n\"a/no-x\" = {}\n") == [
        "rules.lint: 'a/no-x' is listed twice"]



def selftest_upstream():
    def rec(fqcn, in_scope=True, deprecated=False, reason=None):
        return {"fqcn": fqcn, "in_scope": in_scope, "deprecated": deprecated, "out_of_scope_reason": reason}
    rector = {r["fqcn"]: r for r in [rec("R\\FooRector"), rec("R\\OldRector", False, False, "only in php70"),
                                     rec("R\\GoneRector", False, True)]}
    phpstan = {"P\\BarRule": {**rec("P\\BarRule"), "identifiers": ["bar.x", "bar.y"]},
               "PHPStan\\Rules\\Api\\ApiRule": {**rec("PHPStan\\Rules\\Api\\ApiRule"), "identifiers": []}}
    inv = (rector, phpstan)
    good = {
        "rules": {"lint": {"style/no-foo": {}, "style/no-mago": {"origin": "mago"}},
                  "analyse": {"types/bar": {}}},
        "upstream": {
            "rector": {"Foo": {"class": "R\\FooRector", "pun": "lint/style/no-foo"},
                       "Old": {"class": "R\\OldRector", "unsupported": "x"},
                       "Gone": {"class": "R\\GoneRector", "unsupported": "x"}},
            "mago-linter": {"m": {"pun": "lint/style/no-mago"}, "n": {"pun": "lint/style/no-foo"}},
            "phpstan-classes": {"BarRule": {"class": "P\\BarRule", "pun": "analyse/types/bar",
                                            "identifiers": ["bar.x", "bar.y"]},
                                "ApiRule": {"class": "PHPStan\\Rules\\Api\\ApiRule", "unsupported": "x"}},
            "phpstan": {"bar.x": {"pun": "analyse/types/bar"}, "bar.%s": {"pun": "analyse/types/bar"}},
        },
    }
    assert check_upstream(good, inv) == [], check_upstream(good, inv)
    import copy

    def bad(mutate, expect):
        d = copy.deepcopy(good)
        mutate(d)
        out = check_upstream(d, inv)
        assert any(expect in e for e in out), (expect, out)

    bad(lambda d: d["upstream"]["rector"]["Old"].update(pun="lint/style/no-foo"), "only in php70")
    bad(lambda d: d["upstream"]["rector"]["Gone"].update(pun="lint/style/no-foo"), "deprecated upstream")
    bad(lambda d: d["upstream"]["rector"]["Foo"].pop("pun"), "in scope upstream but maps to no Pun rule")
    bad(lambda d: d["upstream"]["rector"].pop("Foo"), "has no [upstream.*] entry")
    bad(lambda d: d["upstream"]["rector"].update(Post={"class": "R\\PostRector", "pun": "lint/style/no-foo"}),
        "not a rule in the pinned inventory")
    bad(lambda d: d["upstream"]["rector"].update(Foo2={"class": "R\\FooRector", "pun": "lint/style/no-x"}),
        "listed 2 times")
    bad(lambda d: d["upstream"]["phpstan-classes"]["ApiRule"].update(pun="analyse/types/bar"), "maps to no Pun rule")
    bad(lambda d: d["upstream"]["phpstan-classes"].pop("BarRule"), "needs exactly one [upstream.phpstan-classes]")
    bad(lambda d: d["upstream"]["phpstan-classes"]["BarRule"].update(identifiers=["bar.x"]), "identifiers differ")
    bad(lambda d: d["upstream"]["php-cs-fixer"] if False else d["upstream"].update(
        **{"php-cs-fixer": {"foo": {"pun": "lint/style/no-foo"}}}), "needs exactly one primary upstream rule")
    bad(lambda d: d["rules"]["lint"].update({"style/no-bare": {}}), "has none")
    bad(lambda d: d["rules"]["lint"]["style/no-foo"].update(origin="mago"), "origin = \"mago\" needs")
    bad(lambda d: d["rules"]["lint"]["style/no-foo"].update(merged=["a", "b"]), "`merged` is gone")
    bad(lambda d: d["upstream"]["phpstan"].update({"bar.x": {"pun": "analyse/types/other"}}), "which no class")
    bad(lambda d: d["upstream"]["phpstan"].pop("bar.%s"), "identifier bar.y")

    def below_floor(d):
        d["rules"]["lint"]["php-8.0/no-foo"] = d["rules"]["lint"].pop("style/no-foo")
        d["upstream"]["rector"]["Foo"]["pun"] = "lint/php-8.0/no-foo"
        d["upstream"]["mago-linter"]["n"]["pun"] = "lint/php-8.0/no-foo"
    bad(below_floor, "below the floor")


if __name__ == "__main__":
    selftest()
    selftest_upstream()
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT
    text = path.read_text()
    dups = duplicates(text)
    if dups:
        print("\n".join(dups))
        sys.exit(1)
    data = tomllib.loads(text)
    errors = check(data) + check_upstream(data, inventory(path.parent / "upstream"))
    for e in errors:
        print(e)
    for c in COMMANDS:
        per = defaultdict(int)
        for key in data["rules"][c]:
            per[key.split("/")[0]] += 1
        later = sum(1 for r in data["rules"][c].values() if r.get("later"))
        print(f"{c}: {len(data['rules'][c])} rules in {len(per)} groups"
              + (f" ({len(data['rules'][c]) - later} for 1.0, {later} later)" if later else ""))
    print("format presets: " + ", ".join(
        f"{p} {sum(p in r.get('presets', []) for r in data['rules']['format'].values())}"
        for p in data["meta"]["presets"]["format"]))
    print("lint sets: " + ", ".join(
        f"{s} {sum(s in r.get('sets', []) for r in data['rules']['lint'].values())}" for s in data["meta"]["sets"])
        + f", no set {sum(not r.get('sets') for r in data['rules']['lint'].values())}")
    print("analyse presets (cumulative): " + ", ".join(
        f"{p} {len(analyse_preset(data, p))}" for p in data["meta"]["presets"]["analyse"])
        + f", opt-in {sum('preset' not in r for r in data['rules']['analyse'].values())}")
    origin = defaultdict(int)
    for tool, entries in data["upstream"].items():
        for u in entries.values():
            if tool in PRIMARY and u.get("pun", "").startswith(("lint/", "analyse/")):
                origin[(u["pun"].split("/")[0], tool)] += 1
    for c in ("lint", "analyse"):
        mago = sum(r.get("origin") == "mago" for r in data["rules"][c].values())
        print(f"{c} sources: " + ", ".join(f"{t} {n}" for (cc, t), n in sorted(origin.items()) if cc == c)
              + f", Mago-derived {mago}, Pun-only {sum(bool(r.get('pun_only')) for r in data['rules'][c].values())}")
    print(f"{path.name}: {sum(len(r) for r in data['rules'].values())} rules, "
          f"{sum(len(u) for u in data['upstream'].values())} upstream entries, {len(errors)} error(s)")
    sys.exit(1 if errors else 0)
