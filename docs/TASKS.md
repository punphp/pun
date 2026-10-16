# Pun task board

Owner = agent working on it. Status: todo / doing / done / blocked.

## Wave 1 (Phase 0 + Phase 1 foundations)

| Task | Crate / path | Owner | Status |
| --- | --- | --- | --- |
| Lexer with token_get_all parity + parity tool | pun-tokens | tokens agent | doing |
| PHP-CS-Fixer Tokens model + Transformers (CT kinds) | pun-tokens | tokens agent | doing |
| php_compat (strlen, strcasecmp, strnatcasecmp...) | pun-tokens | tokens agent | doing |
| Pinned Pint / PHP-CS-Fixer oracle | .oracle, scripts/oracle.sh | harness agent | doing |
| Corpus manifest + clones | corpus/, scripts/corpus.sh | harness agent | doing |
| Differential harness (`pun-harness format`, `--fixer`) | pun-harness | harness agent | doing |
| Laravel preset + Pinkary effective rules + priorities | docs/presets | harness agent | doing |
| Fixer impact ranking | docs/presets/ranking.md | harness agent | doing |
| Upstream fixer fixtures → JSON | pun-harness/fixtures | harness agent | doing |
| pint.json / pun.toml / composer.json (PHP >= 8.4 gate) | pun-config | cli agent | done |
| Pint-identical file discovery, --dirty/--diff/--staged | pun-workspace | cli agent | done |
| Fixer trait, registry, priority pipeline | pun-format | cli agent | done |
| `pun` CLI + `pint` alias, `config --show`, `--engine=pun` | punphp (crates/pun-cli) | cli agent | done |

## Wave 2 (after token layer lands)

| Task | Crate | Status |
| --- | --- | --- |
| Port fixers, split A/B in docs/FIXERS.md | pun-fixers | doing |
| Delegate mode: unported fixers via system PHP-CS-Fixer | pun-format | todo |
| Content-hash cache (blake3) | pun-workspace | todo |

## Deferred (ponytail: system PHP stands in until needed)

- pun-php embedded static libphp (static-php-cli build is heavy on this 2-core box); system `php` used for oracle + delegate meanwhile.
- Phases 2–7 (lint, refactor, migrate, install, run, test).

## Wave 3 (queued — starts when all fixers are ticked and coverage is 100%)

| Task | Status |
| --- | --- |
| `/code-review --fix` over the entire codebase, findings applied | todo |
| Architecture pass: dedupe helpers across fixers/analyzers, tighten crate boundaries | todo |
| Remove comments that restate code; keep only non-obvious "why" (upstream parity quirks) | todo |
| Re-run fixtures, corpus harness and coverage gate after the cleanup | todo |
