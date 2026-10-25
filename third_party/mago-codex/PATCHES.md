# Mago codex 1.57.0: maintained patch

## Source

The library source and normalized manifest come from the published `mago-codex` 1.57.0 package
(checksum `4fad91caa6943ff61d655d1143b864ee265d2e675d0347a9d140ebee6541ba10`). Its
`.cargo_vcs_info.json` identifies upstream revision
[`9656cd55`](https://github.com/carthage-software/mago/tree/9656cd55d51d96bd6dc57f1f1e73c15f22cb7cc3/crates/codex).
Both license files come from that revision. Registry bookkeeping, the package lockfile, external
tests, and benchmarks are omitted, along with their manifest targets and development dependencies.
Sibling Mago dependencies remain pinned to `=1.57.0`. The root manifest patches this crate and
excludes its directory from Pun's workspace members.

## Parent-relative member references

`ttype/expander.rs::expand_member_reference()` backports the codex change from
[`2066b186`](https://github.com/carthage-software/mago/commit/2066b186d2a7cea821e0d4923934032e5f4e32c5),
included in Mago 1.59.0. Before either member lookup or the `::class` shortcut, a case-insensitive
`parent` resolves through `TypeExpansionOptions::self_class` to the class metadata's
`direct_parent_class`. Other names and missing-context fallbacks keep their existing behavior.
No analyzer walk, hook dispatch, native tracking, or dependency version changes.

Public regressions in `pun-analyse/tests/parent_defaults.rs` compare complete diagnostics at levels
0 and 5 with freshly captured PHPStan 2.3.1 output. Distinct ancestor, parent, child, and caller
constants catch incorrect lookup scopes. Controls cover inherited constants and constructors,
mixed-case `parent`, dependent defaults, `parent::class`, `self`, and qualified class names.
`generic_defaults.rs` also checks equal full artifacts and one observer visit per expression with
native tracking enabled and disabled for this fixture. This patch does not close the broader
unspecified-template property gap or change missing-default bounds.

The unchanged 1.57.0 source fails the new public regression: parent defaults become `mixed`,
properties become `int`, and `parent::class` becomes `'parent'`. The patch matches all 24 dumps at
level 0 and all 26 diagnostics at level 5, including two distinguishing argument errors.
Fresh profiles from the six public test binaries cover **7/7 changed codex executable lines**
against the published source and **57/57 changed test executable lines** against `ac32a4b3`,
without exclusions. All other imported codex source is byte-identical to the published package.
The 38-test run uses `cargo llvm-cov --dep-coverage mago-codex --no-report -p punphp-analyse`
with `--test parent_defaults --test generic_defaults --test callback_gather --test debug`
and `--test native_lane --test instantiation`, one Cargo job, and two Rayon threads.
