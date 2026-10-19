#!/usr/bin/env python3
"""Contract checks for generated rule documentation, without changing the checked-in output."""
import importlib.util
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
    output = docs.generate(data)
    lint = output[docs.CONTENT / "rules/lint/style/no-alias-function.mdx"]
    assert 'title: "lint/style/no-alias-function"' in lint
    assert "An alias hides `{intent}` and &lt;meaning>." in lint
    assert "<?php chop($value);" in lint and "<?php rtrim($value);" in lint
    headings = ["## Replaces", "## Turn it on", "## What it does", "### Why", "## Examples", "## Options", "## Linked rules", "## Other rules"]
    positions = [lint.index(h) for h in headings]
    assert positions == sorted(positions)
    analyse = output[docs.CONTENT / "rules/analyse/undefined/method.mdx"]
    assert "**Planned: not in Pun yet.**" in analyse
    assert "Call to an undefined method X::missing()." in analyse
    assert "https://phpstan.org/error-identifiers/method.notFound" in analyse
    ordered = output[docs.CONTENT / "rules/format/imports/ordered.mdx"]
    assert '`"alphabetical"` | `sort_algorithm`' in ordered
    assert "@pun-ignore-file format/imports/ordered" in ordered
    for command, rules in data["rules"].items():
        for key in rules:
            assert docs.CONTENT / f"rules/{command}/{key}.mdx" in output
    for command, rules in data["rules"].items():
        for key, rule in rules.items():
            rid = f"{command}/{key}"
            for fixer in rule.get("fixed_by", []):
                assert docs.rule_link(fixer) in output[docs.CONTENT / f"rules/{rid}.mdx"]
                assert docs.rule_link(rid) in output[docs.CONTENT / f"rules/{fixer}.mdx"]
    with tempfile.TemporaryDirectory(dir=docs.ROOT / ".amp/in") as temp:
        docs.CONTENT = Path(temp)
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
