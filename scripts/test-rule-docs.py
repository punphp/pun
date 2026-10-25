#!/usr/bin/env python3
"""Contract checks for generated rule documentation, without changing the checked-in output."""
import importlib.util
import json
import sys
import tempfile
import tomllib
from pathlib import Path

sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("rule_docs", Path(__file__).with_name("gen-rule-docs.py"))
docs = importlib.util.module_from_spec(spec)
spec.loader.exec_module(docs)


def main() -> None:
    data = tomllib.loads(docs.MAPPING.read_text())
    data["rules"]["lint"]["style/no-alias-function"].update(
        why="An alias hides `{intent}` and <meaning>.",
        examples=[{"before": "<?php chop($value);", "after": "<?php rtrim($value);"}],
    )
    data["rules"]["analyse"]["undefined/method"]["examples"] = [
        {"code": "<?php $x->missing();", "message": "Call to an undefined method X::missing()."},
    ]
    data["rules"]["analyse"]["undefined/method"]["ported"] = False
    output = docs.generate(data)
    lint = output[docs.CONTENT / "rules/lint/style/no-alias-function.mdx"]
    assert 'title: "lint/style/no-alias-function"' in lint
    assert "An alias hides `{intent}` and &lt;meaning>." in lint
    assert "<?php chop($value);" in lint and "<?php rtrim($value);" in lint
    headings = ["## Replaces", "## Configuration", "## What it does", "### Why", "## Examples", "## Options"]
    positions = [lint.index(h) for h in headings]
    assert positions == sorted(positions)
    assert "@pun-ignore-file lint/style/no-alias-function" in lint
    analyse = output[docs.CONTENT / "rules/analyse/undefined/method.mdx"]
    assert "**Planned: not in Pun yet.**" in analyse
    assert "Call to an undefined method X::missing()." in analyse
    assert "https://phpstan.org/error-identifiers/method.notFound" in analyse
    assert '"undefined/method" = "on"' in analyse
    assert "@pun-ignore analyse/undefined/method" in analyse
    for name in ("dump-type", "dump-phpdoc-type"):
        dump = output[docs.CONTENT / f"rules/analyse/debug/{name}.mdx"]
        assert "cannot be suppressed with ignore comments or baseline entries" in dump
        assert "@pun-ignore" not in dump
        assert "Planned: not in Pun yet" not in dump
    ordered = output[docs.CONTENT / "rules/format/imports/ordered.mdx"]
    assert '`"alphabetical"` | `sort_algorithm`' in ordered
    assert "@pun-ignore-file format/imports/ordered" in ordered
    assert "## Examples" not in ordered and "not written yet" not in ordered
    assert "## Linked rules" not in ordered and "## Other rules" not in ordered
    assert "[Group reference](/docs/rules/format/imports)" in ordered
    laravel = output[docs.CONTENT / "rules/presets/format-laravel.mdx"]
    assert '`{ "order" = ["const", "class", "function"], "sort" = "alphabetical" }`' in laravel
    empty = output[docs.CONTENT / "rules/presets/format-empty.mdx"]
    assert "## Enabled rules (0)" in empty
    assert docs.rule_link("format/imports/ordered") in empty.split("## Rules left off")[1]
    for preset in ("laravel", "psr12", "empty"):
        expected = {f"format/{k}" for k, r in data["rules"]["format"].items() if preset in r.get("presets", [])}
        assert set(docs.format_config(data, preset)) == expected
    basic = output[docs.CONTENT / "rules/presets/analyse-basic.mdx"]
    strict = output[docs.CONTENT / "rules/presets/analyse-strict.mdx"]
    assert "levels 0–1" in basic and "levels 6–8" in strict
    assert "union-types = true" in strict and "explicit-mixed = true" not in strict
    assert "explicit-mixed = true" in output[docs.CONTENT / "rules/presets/analyse-max.mdx"]
    tables = json.loads(output[docs.GENERATED / "migrations.json"])["tools"]
    identifiers = {r["name"]: r for r in tables["phpstan"]}
    assert identifiers["method.notFound"]["rules"] == ["analyse/undefined/method"]
    assert identifiers["if.alwaysFalse"]["rules"] == ["analyse/dead-code/number-comparison-always-constant"]
    assert identifiers["if.alwaysFalse"]["url"].endswith("/if.alwaysFalse")
    assert identifiers["phpstanApi.class"]["unsupported"]
    assert all("%s" not in r["name"] for r in tables["phpstan"])
    assert "larastan.jobs.noConstructor" in {r["name"] for r in tables["larastan"]}
    index = json.loads(output[docs.GENERATED / "rules.json"])
    assert "format-empty" in index["selections"] and "naming" in index["selections"]
    directory = index["rules"]
    indexed = {rule["id"]: rule for rule in directory}
    assert len(directory) == len(indexed)
    assert set(indexed) == {f"{command}/{key}" for command, rules in data["rules"].items() for key in rules}
    assert "no_alias_functions" in indexed["lint/style/no-alias-function"]["upstream"]
    assert "StrictArraySearchRector" in indexed["lint/functions/require-strict-array-search"]["upstream"]
    assert "method.notFound" in indexed["analyse/undefined/method"]["upstream"]
    assert "phpstan" in indexed["analyse/undefined/method"]["tools"]
    assert "%s.alwaysFalse" in indexed["analyse/dead-code/number-comparison-always-constant"]["upstream"]
    assert "analyse-max" in indexed["analyse/undefined/method"]["enabled"]
    assert not indexed["format/arrays/no-space-in-empty"]["enabled"]
    assert indexed["lint/laravel-12/prefer-scope-attribute"]["ecosystem"] == "laravel"
    assert "getrector.com" not in analyse
    for tool in docs.TOOLS:
        migration = output[docs.CONTENT / f"migrate/from/{tool}.mdx"]
        assert "## Config translation" in migration and "## Unsupported rules by reason" in migration
        assert docs.rule_link("lint/comments/no-unused-ignore") in migration
    for command, rules in data["rules"].items():
        for key in rules:
            assert docs.CONTENT / f"rules/{command}/{key}.mdx" in output
    for command, rules in data["rules"].items():
        for key, rule in rules.items():
            rid = f"{command}/{key}"
            for fixer in rule.get("fixed_by", []):
                assert docs.rule_link(fixer) in output[docs.CONTENT / f"rules/{rid}.mdx"]
                assert docs.rule_link(rid) in output[docs.CONTENT / f"rules/{fixer}.mdx"]
    with tempfile.TemporaryDirectory() as temp:
        docs.CONTENT = Path(temp)
        docs.GENERATED = Path(temp) / "generated"
        page = docs.CONTENT / "generated.mdx"
        handwritten = docs.CONTENT / "handwritten.mdx"
        handwritten.write_text("Keep this page.")
        expected = {page: docs.page("test", "test")}
        assert docs.sync(expected, True) == 1
        assert docs.sync(expected, False) == 0
        assert docs.sync(expected, True) == 0
        page.write_text("drift")
        assert docs.sync(expected, True) == 1
        docs.sync(expected, False)
        assert docs.sync({}, True) == 1
        docs.sync({}, False)
        assert not page.exists() and handwritten.read_text() == "Keep this page."
    print("Rule docs: page coverage, content, defaults, reverse links and sync checks passed.")


if __name__ == "__main__":
    main()
