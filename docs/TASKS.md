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
| pint.json / pun.toml / composer.json (PHP >= 8.4 gate) | pun-config | cli agent | doing |
| Pint-identical file discovery, --dirty/--diff/--staged | pun-workspace | cli agent | doing |
| Fixer trait, registry, priority pipeline | pun-format | cli agent | doing |
| `pun` CLI + `pint` alias, `config --show`, `--engine=pun` | pun-cli | cli agent | doing |

## Wave 2 (after token layer lands)

| Task | Crate | Status |
| --- | --- | --- |
| Port laravel preset fixers in ranking order, fixtures green | pun-fixers | todo |
| Delegate mode: unported fixers via system PHP-CS-Fixer | pun-format | todo |
| Content-hash cache (blake3) | pun-workspace | todo |

## Deferred (ponytail: system PHP stands in until needed)

- pun-php embedded static libphp (static-php-cli build is heavy on this 2-core box); system `php` used for oracle + delegate meanwhile.
- Phases 2–7 (lint, refactor, migrate, install, run, test).
