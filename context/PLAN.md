# Pun plan

The one file for where Pun stands: how this folder works, every decision, the build order with its status, and the
open questions. Rewritten 2026-10-17 after walking through every command with Pushpak; it replaces README.md,
DECISIONS.md, TASKS.md and REQUESTS.md (their history is in git). Finished work is not listed; `git log` has it.

## 0. Resume here (2026-10-31)

One lead agent works on `main` and spawns subagents for independent slices in worktrees (AGENTS.md "How work runs").
Pushpak resumed all commands on 2026-10-25. CI stays disabled.

- **Counts** (local mapping.toml `ported = true`): format 168 of 168, lint 701 of 701, analyse 306 of 385.
  Pending: format 0, lint 0, analyse 86 (86 total). Ported rules still have the explicit parity gaps below.
- **Callback and unsealed-key integration:** combined main passes 102 analyser tests across nineteen
  complete targets and 169 lint tests across six complete targets. The targeted original-fixture gate
  passes all 274 selected cases in 53 partitions: 126 Methods/ReturnTypeRuleTest cases, 107 PHPDoc cases
  and 41 generics cases. This batch does not replay the broader 405-partition gate below. Existing
  allowances remain unchanged. All 57 CLI tests across six complete targets pass, with parent/child
  profiles matched to seven independently restored runtime objects. All 143 changed executable
  production lines are covered without exclusions; all 26 fresh exact-object exports are warning-free.
  Source-matched coverage remains analyse 44,370/44,371, PHPStan 10,780/10,780 and lint 72,914/73,443
  after existing exclusions. Converter line 643 and 529 lint lines remain uncovered. Four fresh
  PHPStan captures preserve all 49 complete records. The docs build and four rendered HTML text
  contracts pass. All 18,664 frozen source hashes verify, including both symlinks. Evidence is under
  `/tmp/pun-callback-prerequisites-main-gate/`; its restoration manifest preserves all 848 compressed
  closed payloads. Multiple-array activation, source-kind conversion and source identity remain separate.
- **Unset integration:** the frozen main batch passes all 144 analyser tests across 23 complete targets,
  100 lint tests across four targets and all 405 original-fixture partitions. The fixture audit selects
  all 2,787 level cases and 1,315 rule cases exactly once, preserving 623 level and fourteen rule gaps.
  All 57 CLI tests across six targets pass, with parent/child profiles matched to seven verified objects.
  Fresh PHPStan 2.3.1 recaptures preserve all sixty generic-union producer and seven declaration-effect
  records. After integrating the remote union-method lint plugin, all 169 lint tests across six complete
  targets and all sixteen lint CLI tests pass on combined main. The analyser production sources and
  original fixtures remain byte-identical to the frozen gate. All 161 changed executable production
  lines are covered without exclusions; all 35 fresh exact-object exports are warning-free. Current
  source-matched coverage reaches analyse 44,370/44,371, PHPStan 10,780/10,780 and lint 72,914/73,443
  after existing exclusions. Converter line 643 and 529 lint lines remain uncovered; whole-source
  coverage is not complete. All 18,653 current source hashes verify. The docs build and four rendered
  HTML text contracts pass. Evidence is under `/tmp/pun-unset-main-gate/` and its separate
  `/tmp/pun-unset-remote-integration-gate/` replay. Source-identity and its no-op unset adapters remain
  isolated.
- **Constructor producer integration:** the new contracts pass all 89 owner tests on unchanged main.
  Refactored main passes all 270 public tests across eight complete targets and all three library tests.
  All 82 changed executable production lines are covered without exclusions; all nineteen fresh
  exact-object exports are warning-free. Both original property consumers have complete coverage.
  All 10,084 test and fixture files remain unchanged by the refactors. The nine changed sources retain
  seventy uncovered lines, including 47 in property_type.rs; whole-source coverage is not complete.
  Source-matched coverage reaches analyse 44,359/44,360, PHPStan 10,780/10,780 and lint
  72,922/73,403 after existing exclusions. Converter line 643 and 481 lint lines remain uncovered.
  The pinned Rector source hashes and all 18,637 frozen source hashes verify. Changed-file rustfmt
  and diff checks pass. Evidence is under `/tmp/pun-property-producers-main-gate/`, with its separate
  before-refactor gate. Both gates' `compressed-closed-payloads.jsonl` manifests retain restoration
  paths and verified hashes for closed profiles, data and LCOV. Rule mappings and existing expected
  output remain unchanged.
- **Next source-identity prerequisites:** the singleton callable conversion repair is independently
  green and queued, matching all eighty original callable records and twenty mutation controls.
  The separate `array_map` matrix exposes missing production-registry wiring and genuine callback-return
  evaluation; the original `testBug12223` remains green on current main but red on the queued identity
  base. The isolated selected-acceptor prerequisite passes 88 tests across fourteen targets, with all
  140 changed production lines covered, but leaves the fifteen-record public matrix unchanged at two
  matches and thirteen differences. The activation prototype also exposes lost open-list prefixes,
  omitted unsealed-key declarations and a foreign generic fallback on required named-argument holes.
  The isolated omitted-key producer correction passes 349 tests across ten targets and matches all
  nineteen native/value-read records, covering 3/3 changed production lines. Four whole-shape records
  remain red. Fresh open-list controls establish that actual constant shapes and generic lists with
  observed offsets need separate source kinds; blanket shape reconstruction is wrong. The native-kind
  prerequisite is independently green and queued, covering 100/100 changed production lines; its
  combined export retains seventeen external-function warnings. The template-tail trial repairs two
  native bound checks but exposes a closed-shape comparator regression. Invocation inference already
  traverses both prefix and tail. A coherent native validation/comparator repair and the reflected
  array-map fallback are separate worktree tasks. No callable activation or stale-gap removal is integrated.
- **Green prerequisite integration:** the ClassView constructor refactor, generic-array declaration
  effects, ArrayAccess-No offset reads, explicit traversal decisions and additive reflection source
  graphs are integrated on main. All 242 analyser tests across 43 complete targets, 92 PHPStan tests
  across 24 targets, 135 lint tests across eight targets and all 405 original-fixture partitions pass.
  The fixture audit selects all 2,787 level cases and 1,315 rule cases exactly once, retaining the
  existing 623 level and fourteen rule gap allowances. All 63 changed executable production lines
  are covered without exclusions; all 77 fresh exact-object exports are warning-free. Source-matched
  coverage reaches analyse 44,359/44,360, PHPStan 10,780/10,780 and lint 72,881/73,382 after existing
  exclusions. Converter line 643 and 501 lint lines remain uncovered. Four fresh oracle recaptures
  match byte for byte. Reflection metadata adds 779 fields without changing 326,553 existing scalar
  values, retaining all 37,001 effective variants across PHP 8.4/8.5. All 57 CLI tests across six
  complete targets pass, with parent/child profiles matched to seven verified runtime objects.
  The docs build passes on 1,446 pages, with twelve rendered HTML text contracts.
  Source-identity, generic-unset and closure-staticness deliveries remain isolated pending broader
  regressions and the combined source-identity adapters. Constructor consumer and producer-boundary
  refactors are integrated in the batch above. No whole-source parity is claimed. There are no
  remaining `origin/salvage/*` branches. Evidence is under `/tmp/pun-green-prerequisites-main-gate/`.
- **Previous combined integration gate:** all 233 analyser tests across 39 complete targets, 134 lint
  consumer tests across eight complete targets, and all 405 original-fixture partitions pass.
  The fixture audit selects all 2,787 level cases and 1,315 rule cases exactly once. Two obsolete
  level-4 allowances are removed after independent PHPStan 2.3.1 recaptures preserve both complete
  classes: sixty cases and 156 diagnostic records. The current allowance counts are 623 level gaps
  and fourteen rule gaps. All 607 changed executable production lines are covered without exclusions.
  All 49 final exact-object exports and the 101 source-matched cohort exports are warning-free.
  Source-matched coverage reaches analyse 44,341/44,342, PHPStan 10,762/10,762 and lint
  72,884/73,387 after existing exclusions. Converter line 643 and 503 lint lines remain uncovered;
  full parity and whole-source coverage are not complete. The docs build and nine rendered HTML
  contracts pass. All 57 CLI tests across six complete targets pass, with parent/child profiles
  matched to seven independently verified archived runtime objects. CLI profiles do not contribute
  to the coverage counts above. Evidence is under
  `/tmp/pun-config-constructor-effects-main-gate/` and its adjacent `-fixture-gate/` directory.
- **Current disk recovery:** free space is 24 GiB (93% used) after the callback/omitted-key integration
  and concurrent worktree builds. Its gate compresses and independently verifies all 848 closed
  profile, data and LCOV payloads before retiring their raw copies, recovering 3,814,330,368 allocated
  bytes net. All archives, proof sources, worktrees, oracles and active targets remain protected.
  Free space previously reached 35 GiB (89% used) after the combined unset/remote-lint builds.
  The finished unset gate compresses and independently verifies all 1,149 closed profiles, data and
  LCOV payloads before retiring their raw copies, recovering 6,281,576,448 allocated bytes net.
  Its `compressed-closed-payloads.jsonl` records restoration commands and both hashes. Its frozen
  source archive independently verifies all 18,651 source payloads against the original manifest.
  Active targets, worktrees, oracle sources and all proof archives remain protected.
  Earlier on 2026-10-31, free space reached 36 GiB (89% used) after retiring
  328 older generated libraries and compressing 156 inactive test/example executable paths into
  148 unique, independently decompressed and SHA-256-verified archives. This pass recovers
  23,536,934,912 allocated bytes net. Runtime restoration bindings and the rebuildable-cache
  inventory are under `/tmp/pun-disk-recovery-20261031/`. All 18,630 frozen source hashes are
  unchanged; the default target directory, CLI binaries, active gate target, oracles and corpus
  checkouts remain intact. All 766 newly compressed root profiles move outside the worktree;
  `/tmp/pun-closed-source-preservation-20261031.jsonl` records their current archive locations.
  The interrupted fixture gate completes its remaining 107 partitions. After export, another 1,276
  closed profile/data files and 308 CLI profiles have independently verified compressed copies,
  recovering 5,241,712,640 allocated bytes. The current gate's `compressed-closed-profiles-after-gate.jsonl`
  and `compressed-cli-profiles-after-gate.jsonl` preserve their restoration paths and hashes.
- **Disk recovery:** free space recovered from 3.6 GiB to about 37 GiB on 2026-10-30. Active targets,
  worktrees, source changes and oracle installations remain intact. Completed raw coverage profiles
  now have checksum-verified `.profraw.zst` siblings; their restoration manifest is
  `/tmp/pun-compressed-profiles-20261030.jsonl`. Seven retired proof directories are preserved under
  `/tmp/pun-proof-archives-20261030/`, with per-file SHA-256 manifests and full payload verification.
  Another 1,539 completed LCOV/profile-data exports have verified adjacent `.zst` files, recovering
  about 12 GiB; `/tmp/pun-compressed-exports-20261030.jsonl` records their original paths and hashes.
  Sixty-five retired signature executables recover another 7,664,949,427 bytes through verified
  compression; `/tmp/pun-retired-signature-runtimes-20261030.jsonl` records their identities.
  The completed current fixture gate compresses and verifies 434 profiles/data files, recovering
  2,560,946,097 bytes; its `compressed-profiles.jsonl` preserves every original path and hash.
  Restore archived profiles before replaying tools that still use their original paths.
- **Config and offset integration:** mixed/error offsets, non-ArrayAccess object offsets and
  caller-scoped config shapes are integrated locally. All 91 PHPStan tests across 23 complete targets
  and 35 config/Larastan/project/initializer tests across four complete targets pass on combined main.
  All 27 exact-object exports are warning-free. The parent test now compares raw and late operations
  with separately captured PHPStan results, preserving all 28 original raw records. Config captures
  retain all 42 complete records across eight cases, including first-resolution caching per complete key.
  All 88 changed lines are covered on combined main. Generic ArrayAccess
  offsetGet signatures remain unsupported, not replaced with invented reflection. Evidence is under
  `/tmp/pun-config-offsets-integration-gate/` and `/tmp/pun-config-shape-offsets-final-evidence/`.
- **Constructor peers:** genuine typed-peer and inherited helper declarations are integrated locally
  after the shared object-intersection prerequisite. The private four-target gate passes all 119
  tests, including external/generic receivers, namespace-relative names and ambiguous/proven
  Countable intersections. All 153 changed lines are covered privately; whole-owner coverage is
  552/554, leaving two unchanged generic-Node guards. The complete combined lint gate above passes.
  The queued ClassView refactor independently covers all 550 corrected owner lines without exclusions.
- **Public coverage contracts:** the parameter, unavailable-holder and dispatch deliveries pass all
  ten combined-main tests in three complete targets. Their warning-free exact-object exports close
  sixteen old uncovered lines without production edits or exclusions. Source-identical analyse coverage
  previously reached 44,291/44,310. Source-order and callback-dependency contracts now pass the
  combined gate. The current source-matched union above leaves only converter
  line 643 after the existing exclusion: a genuinely red bottom-template class-string contract.
  Its complete failures remain preserved. No failed profile contributes passing coverage.
- **Finally and assertion integration:** actual finally-entry and pending-exit scopes, default-off
  unmatched-catch observations and repeated method-expression assertion refinements are integrated
  locally. All 118 analyser tests across 21 owning and neighboring targets pass, covering all
  234 changed executable production lines without exclusions. All 100 lint consumer tests across
  six complete targets also pass. Flow-edge method-specification joins subsequently pass all 129
  analyser tests across 25 complete targets. Canonical builtin method-effect projection is now
  integrated locally too; its private gate passes both original scanner/queue partitions, all
  56 cases, and covers 20/20 changed lines. The prior seven extra diagnostics remain preserved.
  The complete combined gate above passes every original fixture partition. Both obsolete loop
  allowances are independently recaptured before removal; their original failed logs remain intact.
  Nine fresh PHPStan/Rector
  recaptures and the selected PHPUnit environment are byte-identical. Facts settings and mappings
  remain unchanged. Eight pending-exit values differ between PHPStan 2.3.1 and Rector's embedded
  2.2.7; their independent captures remain intact. General Rector finally parity and the separate
  dead-tail/property-correlated assertion boundaries remain incomplete.
- **Project coverage replay:** all 21 existing tests in `project` and `larastan` pass. Both
  warning-free exact-object exports close seven missing lines, including Larastan loading and
  PHP-version configuration, without source edits, new tests or exclusions. Current source-identical
  analyse coverage is 44,275/44,310, leaving 35 lines across six files. This is not a new whole-suite run.
  PHPStan remains 10,747/10,747 and lint 72,700/73,200 after existing exclusions.
- **Guard coverage replay:** all 23 existing tests across `class_constants`, `dynamic_methods` and
  `pipe_rule` pass. A fresh PHPStan 2.3.1 recapture matches all 287 dynamic-method diagnostic records
  across 22 configurations byte for byte. Three warning-free exact-object exports cover another
  22 previously missing lines without source edits, new tests or exclusions. Current source-identical
  analyse coverage is 44,268/44,310, leaving 42 lines across seven files. This is not a new whole-suite run.
- **Coalesce coverage replay:** all nine existing public tests in `refined_isset_keys` and `coalesce_scope`
  pass on main. A fresh PHPStan 2.3.1 recapture preserves all 96 finite-key records across five levels
  byte for byte. Both exact-object exports are warning-free. Their source-identical union covers all
  476 coalesce lines, including the eight previously missing lines, without production edits, new tests
  or exclusions. Current analyse coverage is 44,246/44,310 after existing exclusions, leaving 64 lines
  across seven files. Lint remains 72,700/73,200 and PHPStan 10,747/10,747.
- **Property-type batch:** the source-owned refactor passes all 52 unchanged `rector_types` tests before
  the behavior correction. Combined main then passes 78 tests across the lint library, engine,
  `rector_strict` and all 64 `rector_types` tests. Twelve fresh Rector 2.6.1/PHPStan 2.2.7 recaptures
  match the complete committed outputs byte for byte. Parent hints retain their relative identity,
  and scalar-only PHPDoc unions use Rector's scalar comparison. All 25 changed executable lines
  are covered without exclusions; the five refactor/final exact-object exports are warning-free.
  Fresh whole-file coverage is 1,158/1,208, with fifty lines still uncovered. Current source-identical
  lint coverage is 72,700/73,200 after existing exclusions, leaving 500 lines. Raw coverage is
  72,707/73,256. The previous property source is not reused. The docs build and two rendered HTML
  checks pass. Changed-file rustfmt passes; the crate-wide check retains unrelated existing differences.
- **Scope and parent batch:** expression-keyed destructuring assignment scopes and unresolved syntactic
  `parent::class` values are verified at f1204856f and delivered with c22d9f75a.
  The complete frozen gate passes 642 Rust tests in all 492 partitions, including every original
  level case (2,787) and rule case (1,315). All 183 owning public/library tests in 33 targets,
  445 native analyser units, 76 targeted CLI tests and 141 lint consumer tests also pass.
  Two fresh PHPStan 2.3.1 recaptures are byte-identical. All 114 changed executable production lines
  are covered without exclusions; all 49 final exact-object exports are warning-free.
  The partition audit selects every original level and rule case exactly once and preserves complete
  oracle records. Source-identical cohorts reach analyse 44,238/44,310, PHPStan 10,747/10,747 and lint
  72,593/73,211 after existing exclusions. The 72 remaining analyser lines and 618 lint lines are
  reported, not excluded. The docs build and four rendered HTML contract checks pass.
- **Combined delivery gate:** the remote lint batches are merged with the local analyser prerequisites.
  Mapping validation passes with 1,256 rules and zero errors; CI remains unchanged.
  The six return-rule contracts have an independently verified 678/678 executable lines and 77/77 functions.
  Two oracle-backed typed-call boundary fixtures are integrated. Callable-array metadata, strict IssetCheck
  native certainty, shared callable selection and dynamic reference scopes are integrated individually;
  the combined runner passes 209 tests in all 36 targets, with seven additional pipe tests, all 76
  targeted CLI tests, 57 config tests and eighty additional lint owner/neighbor tests passing.
  Seven fresh PHPStan recaptures are byte-identical. All 255 changed production lines are covered
  without exclusions; fifty exact-object exports are warning-free. The docs build passes on 1,446 pages
  and five rendered analyse-contract checks pass. Source-identity work remains isolated and red.
  Whole-source coverage is not complete: the source-identical cohort union reaches analyse 43,911/44,209,
  PHPStan 10,747/10,747 and lint 72,593/73,211 after existing exclusions. No new exclusions are added.
  The new complete lint measurement at bd8a29649 passes 628 public and three existing library tests;
  raw coverage is 72,599/73,267, with 668 uncovered lines across 77 files. All 575 source hashes match
  combined main, permitting source-location reuse, not a claim that its old dependencies were retested.
  Its thirty exact-object exports, LCOV and hash manifests are retained. The deleted 98d97e46 report
  cannot establish coverage for the forty files changed afterward and is not reused.
- **Prerequisite delivery gate:** the frozen seven-prerequisite snapshot passes all 2,787 original level cases,
  all 1,315 rule cases and 648 Rust tests in 504 of 505 partitions. The sole failed public target exposes
  a real `break 0` outer-loop subtraction underflow. Its isolated source port matches twelve complete
  PHPStan records and rejects a drop-zero mutant. Corrected main passes all 23 statement tests plus
  132 regression and neighbor tests in 29 targets. Four additional source-matched validation inputs pass.
  A fresh complete PHPStan capture confirms the obsolete `bug-4337.php:72` purity allowance; removing it
  leaves 42 exact purity locations, alongside the unchanged 625 level gaps and fourteen rule gaps.
  All 181 changed prerequisite production lines are covered. The final frozen export has 56 warning-free
  exact-object reports and no unmatched profiles. Corrected-source coverage reaches analyse 44,160/44,169,
  PHPStan 10,747/10,747 and the pre-remote lint source 39,338/39,338 after existing exclusions.
  All five changed outer-loop lines and all 282 owning whole-file lines are covered, bringing changed
  prerequisite production coverage to 186/186. Nine older converter/property lines still need current
  source-matched execution; the queued native-certainty consumer exercises a separate conversion contract.
  A fresh independent boundary also exposes `parent::class` returning Error instead of class-string when
  the current class has no parent. Its complete red records are retained with an isolated source owner.
  The remote lint batch is now merged without discarding the local analyser commits; its mapping is
  696/703, not full parity. Combined config/lint/CLI contracts are being rechecked.
  Its old seven-file conditional-rule coverage report is stale: the exact remote tip passes all 21 public
  quality tests and covers 946/946 owning executable lines without exclusions. Return-type and remaining
  shared-helper coverage audits continue in independent worktrees.
- **Latest verified batch:** native arrays retain insertion order without restoring PHPDoc callable constraints;
  nullable single-class parameters support bounded native nullsafe property reads. Failed reference-array reads
  retain their error/root values, and invalid trait `instanceof` messages describe typed operands.
  The frozen main gate passes 248 public/library tests across 44 targets, 217 original rule cases in
  38 partitions, 197 original level cases in fourteen partitions, 54 coverage partitions and 445 native
  analyser units. Four additional oracle-backed validation controls pass, for 803 Rust tests across
  155 partitions and controls. All 56 targeted CLI tests and forty affected lint tests pass.
  All 64 changed executable production lines are covered without exclusions. All 48 exact-object exports
  are warning-free, with no unmatched runtime profiles. Four fresh PHPStan recaptures are byte-identical.
  The obsolete `$this?->prop` unavailability assertion is removed using independent oracle evidence.
  Source-matched whole-crate coverage is analyse 44,047/44,047, PHPStan 10,747/10,747 and lint
  39,338/39,338 after existing exclusions. Only the existing symbolic-key exclusion moves with its source.
  The docs build passes on 1,446 pages, with ten rendered contract checks. No gap allowances change.
  Completed proof objects are compressed and checksum-verified before their redundant copies are removed;
  source changes, active build targets and coverage evidence remain intact after the disk cleanup.
  The 625 level gaps, 43 exact purity locations and fourteen rule gaps remain. Full parity is not complete.
- **Previous live-initialization batch:** source-owned builtin signature channels, live property initialization markers,
  constructor exit joins, bounded helper summaries and native isset/coalescing transfers are verified on main.
  The 540-partition frozen sweep covers all 782 analyser public/library tests across 135 targets,
  1,315 rule cases and 2,787 level cases. Its original two failures are obsolete native-unavailability
  assertions. Fresh PHPStan captures retain all 58 compound-read diagnostics, nineteen full reads and
  all 23 capture-feedback records, adding nineteen native reads. Both corrected targets pass all five tests
  on main without production changes. Unsupported compound operators remain explicitly unavailable.
  All 56 targeted CLI tests, 88 PHPStan tests and 432 lint tests pass. All-dependency instrumentation passes
  445 native analyser units and 23 public/control tests. Source-matched coverage is analyse 44,020/44,020,
  PHPStan 10,747/10,747 and lint 39,338/39,338 after existing exclusions; all 808 changed production lines
  are covered. All 173 exact-object exports are warning-free, with no unmatched runtime profiles.
  Nine compiler-only profiles are independently matched to procedural-macro libraries and archived separately.
  The docs build passes on 1,446 pages, with eleven rendered contract checks. No gap allowances change.
  The 625 level gaps, 43 exact purity locations and fourteen rule gaps remain. Full parity is not complete.
- **Previous isset/initializer batch:** singleton/dynamic property names, supplied and inherited generic arguments, and
  set-hook writable types now feed IssetCheck. Its advanced setting is independently overridable without
  changing other rules' levels. Nonliteral class-constant initializers retain conditional alternatives and
  array-addition order without selecting a runtime branch.
  The frozen gate passes all 769 analyser public/library tests across 130 targets, 1,315 rule cases and
  2,787 level cases in 520 disjoint bounded partitions. All 56 targeted CLI tests pass, including the same
  forty format comparisons now split across four checkpointable tests. No diagnostic captures or gap
  allowances change. Source-matched coverage is analyse 44,105/44,105, PHPStan 10,740/10,740 and lint
  39,338/39,338 after existing exclusions. Exact-object exports are warning-free with no unmatched profiles
  or new exclusions. The initializer fallback matches four complete fresh PHPStan CLI records byte for byte.
  The docs build passes on 1,446 pages and all six new rendered contract checks pass.
  Completed targets and test executables are removed only after preserving exact-object coverage evidence.
  The 625 level gaps, 43 exact purity locations and fourteen rule gaps remain. Full parity is not complete.
- **Previous reflected-call batch:** compound-method binding, recursive/template mixins, genuine-nullable chains and
  required-method providers preserve declaring-class identity and supplied generic arguments. Object exclusions,
  native argument observations, native-backed function certainty tips, reference bookkeeping, local unset effects,
  refined isset keys, builtin-parent purity and exact unmatched-ignore reconciliation integrate on main.
  The corrected frozen gate passes 765 public/library tests across 129 targets, all 1,315 rule cases and all
  2,787 level cases in 288 disjoint bounded partitions. All 53 targeted CLI tests and all 432 lint tests pass.
  The PHPUnit lint owner exceeds its first 840-second slot; its completed tests are retained and every remaining
  test passes separately. There are no diagnostic failures. All 88 owning PHPStan tests and 445 native analyser
  units also pass. Two obsolete generic-receiver level gaps are removed without changing their captures.
  A fresh mixed-policy generic-proxy regression exposes variance loss during null removal and receiver
  recombination. Its correction matches all thirty oracle records, retaining the original twenty-two; the
  real CLI also matches PHPStan's four complete JSON records. Native unavailability assertions now distinguish
  a real observed int from unsupported continuation. Abstract/interface defaults cover overload availability.
  The abstract-trait control alone leaves one region uncovered; concrete-trait and unrelated-method controls
  cover it while preserving all seven fresh PHPStan purity records. Source-matched coverage is analyse
  43,921/43,921, PHPStan 10,740/10,740 and lint 39,338/39,338 after existing exclusions. All 131 frozen-gate
  exact-object exports and the trait-control export are warning-free, with no unmatched profiles or new
  exclusions. The docs build passes on 1,446 pages, with thirteen rendered contract checks.
  There are still 625 level gaps, 43 exact purity locations and fourteen rule gaps. Full parity is not complete.
- **Current work:** named-argument activation still needs coherent compound semantic signatures, including
  signature-owned defaults/templates and separate native/PHPDoc return channels. Builtin metadata now passes
  the complete combined main gate, preserving all 37,001 existing variant records. The private deferred
  key/value identity and resolution-boundary slice matches fifteen complete query records, passes
  89 PHPStan tests, 220 original rule cases and 440 level cases, and covers 63/63 changed production lines.
  The queued callable assertion carrier matches thirty complete diagnostics, passes 92 PHPStan tests,
  99 analyser tests, 409 original rule cases and 440 level cases, and covers 278/278 changed executable Rust
  lines. Its private compound target improves to six passes/six failures, with 22 missing and no extra records.
  The queued occurrence-position API matches 612 reference records and 25 composition entries, passes
  93 owning-crate tests and covers 85/85 changed lines. The queued source-identity bridge now matches
  all 132 asymmetric TemplateMap combinations, 32 private resolver identity records and 128 complete
  transformation records using their distinct AST-string and Printer channels. Its expanded public
  declaration/annotation suite matches 129 records across 25 configurations. The identity commit covers
  1,361/1,361 changed production lines privately, with independently verified exact-object evidence.
  Its owning gate passes 106 PHPStan tests, 447 native analyser units and 146 analyser tests across
  28 targets. The original Methods rule wrapper passes 409 cases with nine retained allowances.
  A subsequent original Methods level run on that unchanged bridge base reports six stale-gap
  entries and a list/array mismatch. Fresh PHPStan 2.3.1 captures confirm all 571 cases and 2,787
  complete diagnostic records in their containing Methods/callable families, including all six stale
  entries. The genuine `testBug12223` mismatch requires the missing non-null, multiple-array
  `array_map` return-provider branch, not weaker list compatibility. A callable neighbor retains
  22 actual versus 23 expected records: 21 matches, two missing and one extra bottom type. The
  fresh main control passes all five callable families. The identity branch exposes an unintended
  singleton union normalization during conversion; its source-owned correction is in progress.
  These are not green whole-rule results.
  The separate closure-staticness prerequisite covers 36/36 changed lines and fixes one extra
  empty-offset diagnostic, but the original expanded purity replay remains red. Actual closure purity,
  void authority and compound activation remain incomplete. Do not import the private red ancestry.
  The independent builtin callable factory now preserves every source variant, matching 112 complete
  records and covering 23/23 changed lines privately; it is integrated locally with combined verification running. Void authority remains
  unfinished. The compound guard and mapping stay inactive.
  Isset activation still needs native certainty configuration. Constructor exit-scope initialization and
  bounded helper selection now pass the complete combined main gate. Tied distinct declarations and
  ambiguous carriers remain conservative.
  Copied class-constant ordering has a verified private correction with 76 complete oracle records.
  Private canonical conditional array addition matches 127 diagnostic records; its exact-key
  converter correction now matches all 92 same-value records and forty unchanged narrowing/unset controls,
  passes 91 tests and covers 5/5 changed production lines. The queued public 256-alternative fallback preserves
  keys, values, nonemptiness and guaranteed offsets, matching 259 complete records and passing 93 tests with
  62/62 changed production lines covered. Native-only array insertion order passes the main gate above.
  Native isset/coalescing carriers now pass the complete combined main gate, including assignment handoff
  alongside marker writes. Their original 201 supported observations and 26 unavailable controls remain.
  The strict certainty consumer is now independently committed and queued: all 108 complete comparisons
  pass, covering 888 diagnostic record instances through both constructors without filtering. Its seven
  native observations and 29 once-only events remain unchanged. It requires the queued callable-array
  metadata prerequisite plus the locally integrated generic-property and coalesce-purity corrections.
  The separate generic property correction matches twelve complete comparisons and covers 6/6 changed lines.
  The local coalesce purity correction matches 90 oracle records and covers 36/36 changed lines; its private
  consumer replay corrects 24 record instances but does not make the full consumer green. Both commits
  are integrated locally with combined verification running. The callable-array metadata prerequisite now
  matches twelve complete comparisons and covers 6/6 changed lines privately. Its consumer replay fixes
  the two description locations while retaining deleted offsets and native observations. The queued strict
  consumer covers 20/20 changed production lines and one required test-initializer line; no native/PHPDoc
  fallback is accepted. Combined-main integration remains.
  The integrated parameter clamp changes exactly three native-nullsafe
  support flags, preserving every full/native oracle value. The owning nullsafe target passes with 48 supported
  observations, thirteen unavailable controls and 165 once-only visits. Its broader combined gate remains open.
  The parameter-clamping correction matches all 91 diagnostic records and covers 31/31 changed lines.
  Native ordering retains 79 complete records and the 38-record consumer
  control without the twelve callable false positives. Reference-array reads match 43 diagnostics and
  48 observations on main; their broader missing-offset diagnostic still needs the level-3 read rule.
  Boolean-membership narrowing has a locally integrated correction matching nineteen complete records through both
  constructors, passing 53 tests and covering 79/79 changed lines without changing its 31 gap entries.
  Two empty-offset prerequisites are integrated locally: all-level reachability and coalesce target scopes.
  They cover 6/6 changed lines, but the whole-rule probe still has five extra records (37 versus 32).
  Queued callable-reference transfers pass 64 final-source tests and cover 42/42 changed lines, reducing
  the whole-rule probe to 33/32 records. Sequence-sensitive destructuring remains with its independent owner.
  The mapping stays inactive. The seven-prerequisite initial combined gate passes all fifteen owning/library tests;
  seven fresh PHPStan captures are byte-identical. All 181 changed executable production lines are covered,
  with 54 warning-free exact-object exports and no unmatched profiles. All 445 native analyser units pass.
  The expanded 505-partition gate finishes with 504 passing partitions and the isolated low-level `break 0`
  failure. PHPStan's fresh complete 105-record statements capture matches unchanged expectations. Its source
  correction passes all 23 statement tests and 132 additional public/library tests on corrected main.
  All 56 targeted CLI tests and forty affected lint tests pass. Whole-file coverage has nine analyser lines to reach;
  PHPStan and the frozen pre-remote lint source retain complete source-matched coverage.
  The docs build passes on 1,446 pages, with twelve rendered DOM contract checks.
  Completed proof executables are compressed and restored hashes verified
  before retirement; active targets, sources, captures and coverage exports remain intact.
  One independently verified obsolete purity allowance is removed. This local batch is not pushed yet.
  A fresh class-string template control retains a separate description gap: Pun prints `T of *NEVER*`
  where PHPStan prints `T`. Its original failure is retained, without a new gap allowance.
  CI stays disabled. Next: finish the combined prerequisite gate before integrating the strict consumer.
- **Preceding verified batch:** overflowing integer literals retain their PHP magnitude and radix-specific
  rounding, including values beyond `u128`. Fresh PHPStan captures match all 45 diagnostics and 171 literal
  kind/value/raw-byte/span records. All 393 combined partitions execute: 1,315 rule cases, 2,787 level cases,
  551 analyser public/library tests, 431 lint tests and all 78 owning PHPStan tests. The first gate finds one
  sorted-integer-keys regression; Mago's lexical integer contract is preserved, then all 34 affected tests
  pass in native and instrumented builds with fresh radix, clamping and ordinary-float controls. The other
  392 partitions pass before that bounded lint-only correction. Existing expectations and gaps are unchanged.
  Source-matched coverage is analyse 42,846/42,846, lint 39,271/39,271 and PHPStan 10,557/10,557 after
  existing exclusions. All 50 changed executable lines are covered without new exclusions. All 51 analyse,
  lint, migrate and init CLI tests pass. The docs build passes on 1,446 pages, with all three new rendered
  contract checks passing. Coordinator timeouts retain successful partitions against frozen source hashes;
  original failures, interruptions and logs remain available. Normal coverage export succeeds, with the same
  18 warnings for unchanged inline flag methods as the previous batch. There are still 653 level gaps and
  16 rule gaps, so this is not full parity. The preceding format gate matches 20 Pint CLI comparisons and
  all 799 Pinkary files; its absent-dependency analyse errors are not analyse/check parity evidence.
- **Completed integration history:** shared joins, non-plain empty-array alternatives, duplicate constant reflection,
  forced late-shape returns, reflected call policy, trait-alias identity and immediate-interface selection are
  integrated locally. The main pilot has no new diagnostic mismatches. Two obsolete rule gaps are removed
  after the complete Methods and Classes sweeps pass their expectations. The 394-partition combined gate is
  split at the 900-second coordinator budget, retaining frozen-source checkpoints. Functions level 5 identifies
  one obsolete gap; level 9 identifies two obsolete gaps and one real callable-after-branch regression. The
  full sweep also finds an impure-points union-declaration display regression. After dividing the unchanged
  126-case Methods return fixture into three verified 42-case partitions, 395 of 396 partitions finish. The
  final return partition still exceeds 900 seconds; its large sequential optional-array fixture needs a
  source-owned join reduction check. No expectations or gap allowances are weakened. Mixed-case alias
  spelling and reproducible codex unit prerequisites are now integrated locally, exposing one existing
  callable/truthy-mixed union failure; the full unit target is not green yet.
  PHPStan's reverse if-branch merge order fixes the impure-points regression: all seven fresh public
  diagnostics, the unchanged six-case original partition and 30 neighboring tests pass. The callable and
  slow return failures remain with independent source owners. Four proven obsolete level gaps are removed,
  leaving 649 level gaps and 14 rule gaps locally.
  Independent argument-type/reference and receiver-eligibility settings are integrated locally. The private
  source port matches 639 complete records across 30 configurations; the integrated main snapshot passes
  all 49 affected analyser tests and all 57 config tests. Arbitrary Pun TOML overrides still need option
  ownership, so migration rejects them with a precise reason.
  All 51 affected CLI tests pass. The 20 owning PHPStan targets pass before interface integration.
  Independent call, override, consistent-constructor and final-join registrations are also integrated locally;
  their private public gate matches 37 complete records and all 16 CLI owner combinations. The integrated
  settings/registration snapshot passes all 52 affected public analyser tests and all 52 affected CLI tests.
  All eight affected original-fixture partitions pass against unchanged source: 409 Methods rule cases,
  11 Traits rule cases, 220 method-call cases each at levels 5 and 9, and four coverage-fixture filters.
  Existing allowances remain unchanged. Fresh whole-crate coverage remains. Crate-wide rustfmt checks still
  report formatting drift, also reproduced in the parent snapshot; both new registration test files pass.
  The docs build and thirteen rendered contract checks pass. Named-argument activation still needs per-name
  filtered scopes and complete configuration boundaries. The dynamic-name probe also fails for a literal
  call inside an equality branch: shared inference discards differing target types when branch certainty agrees. That prerequisite
  and callable constraints have independent source owners. Fresh PHPStan probes also show that filtered reads
  use each operand's own pre-evaluation scope, including earlier argument writes and impurity invalidation.
  A source-owned opt-in observation facility is assigned before the checker bridge; a single filtered
  call-entry snapshot is insufficient. Conditional holders now integrate locally with reverse if-edge
  order preserved. The private correction matches 207 complete records and covers 160/160 changed
  executable lines; all 46 affected public analyser tests pass on main. Property receiver eligibility also
  integrates locally: its private 12-case matrix matches 123 records through both Project constructors,
  with 8/8 changed lines covered, and its controls pass in the same main gate. Template-intersection
  array/accessory views now integrate locally, matching 78 new records and all 80 owning-crate tests.
  The docs build passes on 1,446 pages, with nine rendered contract checks passing.
  The requested-name API is integrated locally and all 13 affected main tests pass. The live receiver
  correction now matches all 42 expanded PHPStan records and the two-record scanned-stub purity control,
  including inherited aliases, native versus PHPDoc returns, constructors, readonly guards, conditional
  holders, private-parent properties, compound clauses and ordinary/nullsafe receiver guards. The combined
  instrumented gate passes 28 tests across eight targets, with 32 more tests across nine API-neighbor targets.
  Fresh exact-object profiles cover 163/167 receiver/API/join executable lines since template-view integration,
  without exclusions. Remaining API and receiver-effect paths still need coverage before delivery.
  The private observation facility passes all seven public facility tests and 197 strict observations,
  retaining the original 155 unchanged. Its broader gate passes 85 public tests and two source-contract tests.
  Real event-count, live-artifact and diagnostic-isolation assertions pass; semantic providers remain active.
  Changed coverage is 612/620 without exclusions. The eight remaining paths are assigned before integration.
  Same-key array discrimination now matches all 58 complete oracle records and 58 neighboring public tests.
  Fresh coverage from the exact instrumented regression executable covers all 5 changed executable lines
  without exclusions. This restores sibling-offset correlations while retaining upstream's one-field merge.
  The integrated sequential-join commit completes unchanged bug-12110 in 6.200 seconds with no diagnostics
  versus a 40-second parent timeout. It matches all 36 sequential and 32 ordering records, eight template
  controls and both direct/deferred offset-order observations. Its private gate passes 76 neighboring
  tests and all 80 owning PHPStan tests, with 34/34 changed executable lines covered. On combined main,
  all 17 targeted analyser tests, all 80 PHPStan tests and the complete Methods level sweep pass.
  That sweep takes 613 seconds. Fresh combined profiles cover all 35 changed join lines and three offset
  traversal lines. The docs build and twelve rendered contract checks pass. The private filtered named-call
  regression retains all thirty level-5 oracle diagnostics and empty level-4 controls. Main still has three
  missing/one extra records in the original level-5 probe, twelve missing/two extra in its added boundaries,
  and one extra argument.duplicate in the level-4 identifier/later-impurity control. The checker bridge must
  preserve argument evaluation positions for ordinary binding too. No expectations or allowances are weakened.
  Receiver-guard conversion now accepts an explicit expression-type map; all twelve affected public tests
  pass instrumented with unchanged expectations. The selected array-key dependency refactor is integrated;
  its private gate passes 92 unchanged public tests and covers 61/61 changed executable lines. The remaining
  conversion paths have a separate source owner and must use the same map for literal children, provenance
  and guards. Callable-array constraints and the truthy-mixed combiner correction are also integrated locally.
  Their private gate matches all 80 frozen oracle records, passes 298 codex units and 101 public/library tests,
  and covers 530/530 changed executable lines. Its Functions level-9 partition has no new failures, but reports
  two obsolete gaps already removed on main. All 80 owning PHPStan tests pass on combined main; the combined
  analyser integration gate passes all 21 tests across seven targets. The subsequent receiver-edge gate
  passes all 28 tests across eight targets, including 45 fresh receiver records. The Functions sweep finds
  zero new failures and one newly obsolete level-5 bug-12847 allowance, now removed. Eleven analyser unit
  failures also occur on the exact callable base. The observation facility is integrated locally, with
  its coverage correction and external requested-name worker regression. The corrected private facility
  covers 631/631 changed executable lines without exclusions, and all 15 worker-backed method-call tests
  pass. Nullsafe ensured-scope support remains with its source owner. The checker bridge compiles, but its
  frozen complete-record regression still has one passing and three failing tests after separating logical
  call spans from diagnostic selectors and requesting level-4 binding scopes. Fresh oracle recaptures remain
  byte-identical. The broader eight-target gate passes 42 tests and fails seven renamed-argument tests,
  exposing missing nullsafe answers, template/mixed selector extraction, and historical source dependencies.
  The parenthesized abstract-constructor receiver regression now passes its unchanged public test after
  restoring the consumer's receiver unparenthesizing. Temporary diagnostics confirm that requested earlier
  receiver spans are absent from selected maps; template/sealing metadata cannot be restored safely without
  them. Those facility boundaries remain with their source owner. No global-type fallback is added.
  Written argument evaluation order now passes all 45 owning public tests on combined main and covers
  25/25 changed executable lines without exclusions. Both fresh PHPStan unpack captures match byte for byte.
  The focused consumer gate improves to two passes and two failures: only the nullsafe call and the two
  assignment-derived correlation records are missing, with no extra diagnostics. The existing collector
  target passes 14 tests and fails six at its known nullsafe, selector and historical-dependency boundaries.
  The docs build passes on 1,446 pages and all ten rendered contract checks pass. Independent source owners
  continue assignment-derived conditional holders and finite array-key literal precision. No selected scope
  uses the global type map. After the unsplit Functions sweep exceeds 900 seconds, all nine bounded level
  partitions pass on the same frozen production snapshot, covering 693 cases. Full combined verification
  remains open.
  Bounded finite-key array literals integrate locally and pass their frozen 50 live-type contracts and
  five complete level-5 diagnostics. The private gate passes 66 public tests with 188/188 changed production
  lines covered. Fresh recapture is byte-identical. On main, all 28 tests across the observation, finite-key,
  shared-join, sequential-join and shape-discrimination targets pass. Nullable nested-array joins reproduce
  two raw-artifact failures before the bounded correction, retaining disjoint-field controls in both orders.
  The final callable/list-guarded source passes the same 28 tests. Exact-object profiles cover all 30 changed
  join lines and all 188 changed array-builder lines without exclusions or export warnings. The docs build
  passes on 1,446 pages, with all fourteen rendered contract checks passing. Full coverage remains open.
  The unfinished private nullsafe port preserves 228 earlier observations and matches its 24-record primary
  probe, but three facility tests still fail on seven method-holder, copied-holder, nested-array or void-call
  records. Its changed coverage is 350/354 without exclusions. None of that WIP is integrated. A fresh
  historical-dependency capture independently confirms all 15 records. Selector extraction and historical
  retention now integrate locally. Their private gate passes 18 facility tests and covers 126/126 changed
  production lines without exclusions. With the exact consumer overlay, renamed-argument tests improve
  from 14 passes/six failures to 16 passes/four nullsafe-only failures. The focused consumer still has
  three missing records on that private base. On combined main, all nineteen facility tests pass and
  exact-object profiles cover all 126 changed selector/history lines without exclusions or export warnings.
  The focused consumer passes three tests and fails one, with only calls.php:94 missing and no extras:
  both assignment-correlation records now match after finite-key integration. The existing collector passes
  sixteen tests and fails four at its known nullsafe boundaries. Fresh selector/history captures remain
  byte-identical. No global type-map fallback is added. Full combined verification and coverage remain open.
  The receiver/API coverage gate now covers all 197 changed executable lines without exclusions or export
  warnings. Seven receiver/API controls, the real PHP worker's requested-name control, seventeen array
  neighbors and a deferred-intersection offset control pass. Combined profiles reuse earlier observations
  only for byte-identical source hashes. This is changed-line evidence, not fresh whole-crate coverage.
  The assignment matrix's missing isset.variable is a separate unimplemented IssetRule invocation: the
  existing IssetCheck serves coalescing only. The lead owns its diagnostic invocation and operand observations;
  the assignment owner retains the strict complete-record matrix and source-owned intersection prerequisite.
  A fresh serial CLI build does not reproduce batch 6's reported private-method deletion: all three Pinkary
  outputs match Rector 2.6.1 byte for byte, with receiver types present before and after the facade rewrite.
  Isolated IncrementViews runs also match Rector with default concurrency (RAYON_NUM_THREADS unset) and
  one process, for both the canonical input and an older whitespace-only variant. The reported deletion
  remains a verified older combined-tree failure, not a reproduced current-main bug. That older source
  object is unavailable, so its precise cause remains unresolved. The test-only multi-round guard passes
  all six dead-method tests and the facade control on combined main. No production correction is applied.
  Raw/value call-result storage is integrated locally. Its private source gate covers 336/336 changed
  executable lines and reproduces the same ten pre-existing library failures on its unchanged base.
  Main passes all nineteen existing facility tests and, after reconciling ordinary value getters with
  the pinned oracle, all nineteen native-lane tests. Native void remains explicitly unavailable in the
  partial native API. The lead is wiring snapshot transport and Checker keep-void consumers. A fresh
  mixed-arm match control exposes generic union normalization changing 1|void to 1|null; match arm
  and invocation-target unions now preserve void in the shared combiner. Only eligible source-call
  result reads project void to null. No combiner or analyser setting selects that syntax-dependent rule.
  Named-argument gates still have one focused and four collector failures at the private nullsafe boundary.
  No expectation or existing gap allowance is weakened. Full combined coverage remains open.
  Isset invocation, assignment-derived consequences, definite constant-array intersections and the
  nullsafe live-scope prerequisite now integrate locally. The private assignment matrix matches all
  200 records across 24 configurations with the exact isset source and lead dispatch. Its intersection
  controls match 78 results in both operand orders and cover 234/234 changed production lines.
  The private nullsafe prerequisite matches 57 type observations and seven complete diagnostics,
  passes 69 targeted tests and covers 263/263 changed production lines. Observation registration and
  historical nullsafe-root pricing remain separate work. Main's consumer gate passes twenty facility,
  nineteen native-lane, five incoming-isset and three coalescing tests. Its new void diagnostic matrix
  still misses a constant dynamic method call, whose ordinary rule currently skips dynamic selectors.
  That source branch has an independent owner. Fresh PHPStan probes also distinguish raw int|void
  invocation-target unions from named-call int|null values; the expanded transport regression is running.
  Fresh matrix, match and twelve-marker transport recaptures are byte-identical, including actual
  function-visit counts. The integrated prerequisite build completes in 767 seconds, but its 900-second
  command budget expires inside the assignment matrix before a result. The cached retry executes all
  54 tests: 52 pass, including the complete assignment, intersection, conditional-holder, native and
  shape targets. One transport assertion exposes raw callable void being treated as null by coalescing.
  One nullsafe diagnostic misses the independently owned dynamic-method branch. Explicit-null removal
  now preserves void, with sixteen frozen transport controls under verification. All twelve earlier
  transport records remain byte-identical. The timeout and failing assertions remain separate evidence.
  The combined dynamic-method and transport gate now passes all 24 facility controls, four dynamic-method
  tests and five argument-contract tests. Its sixteen transport observations match fresh byte-identical
  PHPStan recaptures after ordering the new fixture's named keys consistently with the existing fixtures;
  no comparison is relaxed. Native, generic-default and coalescing neighbors also pass. The complete
  six-target consumer gate passes 54 tests and retains six nullsafe-only failures: one focused collector,
  four existing collector modes and one nested dynamic void-use diagnostic. Nullsafe observation work
  remains private, and the mappings stay inactive. The docs build and all fifteen rendered contract
  checks pass. Full combined verification and coverage remain open.
  The refined isset offset gap also needs a decimal-integer-string producer: `(string) $int` currently
  becomes numeric-string. Its frozen ten-configuration capture retains 59 complete records, and the
  producer owner is porting the representation before shared offset coercion. The mapping stays inactive.
  The first fresh combined build finds a stale test-only Checker initializer; its six missing fields now
  use level-0 defaults. The first bounded level-9 Methods partition finds two new false positives:
  dynamic method_exists checks lose their original-selector guarantee, and function-entry conditional
  parameter relationships never reach selected-name pricing or closure captures. Both level-5 and level-9
  void-conditional allowances are proven obsolete and removed, leaving 647 level gaps. Strict nullsafe
  observation tests expose fifteen absent assignment-target answers, six absent cached-property receiver
  answers and duplicate argument evaluation for multiple method targets. Those producer boundaries have independent owners. The private
  nullsafe consumer passes four focused and twenty collector tests, but its facility and public diagnostic
  targets still fail. No incomplete subset is treated as parity or pushed.
  Original-selector method_exists guarantees now match all 55 frozen records across three levels, including
  first-call impurity invalidation and early-return continuation. All five argument contracts and three
  nullsafe diagnostic tests pass in that gate. Expanded dynamic nullsafe controls still miss one diagnostic
  before observation registration. Nullsafe registration and its strict facility tests now integrate as
  local WIP. The main six-target gate passes 63 tests and fails six: the same five facility producer
  boundaries and one dynamic void-null control. All four focused consumers, twenty original collectors,
  five argument contracts and three original nullsafe diagnostics pass. The expanded void-null control
  now finds the previously missing record but exposes two false positives from short-circuit provenance.
  All six frozen nullsafe observation captures match PHPStan again, including method-visit counts.
  The docs build passes on 1,446 pages. Full combined verification and coverage remain open.
  Decimal-integer string production and array-only key coercion now integrate locally. Their private
  gate matches 95 complete diagnostics and covers 95/95 changed production lines. The sequential
  string-offset probe still exposes an unrelated coalesce placeholder/holder identity error, now with
  a source owner. Corrected unpack-overload selection also integrates: all 120 private oracle records
  match, retaining the earlier 114 unchanged; unknown unpacks overwrite collided keys while constant
  alternatives union. Main reproduces shared mixed subtraction's missing exclusion before correction,
  with 65 fresh public type-API records and every earlier oracle record unchanged. The nine-line
  shared correction passes all 298 Codex unit tests. Native condition transport and the PHPDoc certainty
  tip are under verification against eight fresh complete records; the unchanged baseline misses exactly
  two tips. Both focused main targets now pass all thirty tests, with all six changed executable
  subtraction lines and all seven changed If-consumer lines covered without exclusions or export warnings.
  The eighteen-target aggregate completes eight targets before its 900-second budget: 71 tests pass and
  five facility tests fail at the known prepared-read and repeated-invocation boundaries. The interrupted
  conditional target is not counted as complete. All 14,247 source hashes remain unchanged for that run.
  Prepared assignment/property positions and declared-null return provenance now integrate locally.
  The private prepared port restores all 237 frozen answers and covers 229/229 changed production lines.
  Conditional formal holders and read-only reference feedback also integrate, with 59/59 changed lines
  covered privately. Their type records and original bug-12875 match after the shared subtraction port.
  Strict remaining diagnostic producers are If purity and unused-capture value flow. The latter's 158
  fresh records expose semantic exception, alias and nested-scope requirements, now assigned to a source
  producer instead of a lexical workaround. No failing expectations are removed. Combined verification
  of these integrations remains. The docs build and the three new rendered contract checks pass.
  The frozen seventeen-target gate now completes: 141 tests pass and five fail across three targets.
  All 220 unchanged original level-9 method-call cases pass. Exact-object coverage exports all eighteen
  built executables and twenty-one fresh profiles without warnings, retaining the failing results.
  Prepared positions and conditional types pass; remaining failures are repeated argument evaluation,
  ten declaration-purity/two unused-capture diagnostics, and an original nullsafe receiver diagnostic.
  The branch-effect source delivery now integrates locally, with its private 138 complete records and
  79/79 changed lines covered. A dedicated original-receiver artifact is under combined verification;
  it must distinguish the same nullable receiver from independent non-null and assigned receivers.
  Main now passes the complete branch-effect, declaration-purity and dead-code targets. The parameter
  matrix improves from two to four passes, retaining only the two unused-capture diagnostics.
  The first receiver-artifact control exposes missing function-body transport; that transfer is now
  explicit. Coalesce holder identity and absent-offset certainty also integrate locally: the private
  source gate matches 126 new diagnostics, 26 holder snapshots and sixty reads in each native mode,
  passes 39 tests and covers 82/82 changed executable lines. Combined verification remains open.
  The corrected seventeen-target gate completes on unchanged source: 95 tests pass and three fail.
  All eight original nullsafe-method cases and six expanded dynamic-method controls pass. The three
  strict remaining failures are a repeated argument visit, two unused-capture diagnostics and an
  absent-offset coalesce assignment widened from 5 to mixed after prepared-target integration.
  The unsplit original level-9 method-call partition passes all 220 cases; the 900-second coordinator
  interrupts the return and level-5 partitions, which are not counted as complete. Exact-object
  coverage exports seventeen executables and eighteen profiles without warnings. Half-open case
  bounds are being added to the level harness so large fixtures can run without rewriting captures.
  The multiple-target invocation candidate fixes the repeated visit but still loses one union-bound
  closure-this type and has uncovered paths; it is not integrated. Unit-failure classification also
  separates stale NoValue/termination expectations from genuine ErrorType, interface-result and
  foreach-certainty consumers. Independent source owners retain those strict boundaries.
  The foreach certainty and direct-property target corrections pass their original unit controls.
  Together with the ErrorType assignment delivery, the fresh unit gate improves from 432/443 to
  441/444. The remaining interface and two Closure::getCurrent failures are not accepted as inherited
  failures. The interface signature delivery and frontend argument-check continuation now pass all
  four strict public interface controls. The ErrorType assignment regression and four prepared-read
  controls also pass on main. A bounded 42-case return partition and 40-case level-5 method-call
  partition pass without changing any oracle record or gap. Exact-object profiles export without
  warnings. The semantic capture-flow delivery is integrated, preserving native and original-receiver
  transport; all fourteen combined parameter and capture tests pass. The coalesce integration gate also
  passes all nine tests, including the original absent-offset assignment and strict holder observations.
  Fresh getCurrent captures retain all
  55 existing producer records and add six implicit-never/ErrorType records plus three continuing
  statement controls. Their baseline fails on the missing dynamic-call result. The once-only invocation
  and mixed-addition deliveries now integrate locally, preserving the actual-walk capture-reference hook.
  Their private gates pass 154 call tests and 44 addition/neighbor tests, with 618/618 and 22/22 changed
  production lines covered without exclusions. The native return consumer independently reproduces
  rejected benevolent int returns; its correction passes all sixteen public Analyzer combinations,
  retaining strict-union and incompatible-return controls from fresh PHPStan acceptance results.
  The current frozen fifteen-target main gate passes all 141 test functions, including all 36 facility
  tests, fifteen once-only invocation tests, nine capture-flow tests and five conditional-parameter tests.
  Fresh standalone units pass 423/444: all 21 remaining failures expect ImpossibleAssignment, but fresh
  PHPStan proves their constant, instantiation and first-class callable producers also return incorrect
  never values. Three independent source corrections retain strict public value and diagnostic controls;
  no warning is restored and no expectation-only cleanup is accepted. All 120 integration executables
  link, and the cached library build succeeds with all six units passing. The frozen level sweep executes
  all 2,787 cases in 226 partitions. Thirteen partitions fail: obsolete case/purity allowances account for
  all but two new diagnostic regressions, nested array reference writes and a generic-array compound
  assignment whose prepared input is overwritten by the write. Both have source owners. The public gate
  also exposes duplicate class-constant editor targets after memoized receiver evaluation and three false
  void-use cascades for inaccessible callables. Original captures stay unchanged. Rule partitions now
  select individual fixture files after the full Methods group exceeds the coordinator budget. Public
  and rule sweeps continue against the same frozen source hashes. Full combined verification and coverage
  remain open; the coalesce written-holder delivery is verified privately but not integrated yet.
  The rule sweep also exposes a DatePeriod missing-parameter description narrowed to int instead of
  int|TEnd of DateTimeInterface, and a foreach-correlated dynamic method checked against the wrong name.
  The failed-method public control distinguishes genuine unknown mixed/object callees from invalid
  callees: the former must retain mixed rather than the new invalid-call ErrorType fallback. All three
  regressions remain exposed. Fresh callable visibility captures reproduce private/protected signature
  leakage while preserving accessible calls and escaped closures. Whole-crate coverage export now succeeds
  with an explicit primary object; the unfinished public sweep and owning PHPStan tests still need to
  contribute before its counts are complete.
  That frozen sweep is complete: 118 public targets execute 703 tests, with 699 passing and four failing;
  all six library tests pass. The rule sweep executes all 1,315 cases in 143 partitions, with only the
  DatePeriod and foreach partitions failing. The owning PHPStan sweep passes 79 tests and fails one
  because its fixture decoder lacks explicit-mixed; that decoder is corrected locally. Coverage before
  these follow-ups is analyse 43,468/43,532 and PHPStan 10,552/10,565 after existing exclusions, with no
  export warnings. This is not 100% coverage. Eleven proven obsolete level gaps and eight exact purity
  locations are removed, leaving 636 case gaps and 43 exact purity locations. Original captures remain.
  Coalesce written-holder certainty now integrates locally, with its private 60-test gate and all 87
  pinned diagnostic records retained. Memoized receiver editor identity, unconditional overload operand
  requests, unknown dynamic callees and inaccessible callable results have local corrections. Fresh
  captures retain all 120 earlier overload records and add four constructor records per configuration;
  the callable matrix has 36 complete records. The combined instrumented main gate passes all 47 tests
  across eight targets, including negative-callable mixed, native capture availability and the integrated
  nested-reference write correction. All four owning PHPStan mixed-subtraction tests and all 298 codex
  units pass. The standalone codex gate uses both maintained parser patches. Full combined fixture and
  coverage gates remain open. All thirteen previously failing level partitions are rerun: twelve pass,
  leaving the compound-input producer regression. The first-class-callable delivery passes thirty public
  controls with 19/19 changed production lines covered, but exposes a native bare-Closure argument
  compatibility failure. That source correction remains assigned without changing the failing assertion.
  Independent worktrees retain the compound-input, failed-constant, instantiation and foreach-holder
  corrections. Restored receiver operand publication also permits removal of historical receiver and
  synthetic `$this` fallbacks; their combined main gate is running. The cancelled mixed-object coverage
  slice contributes no tests or source changes. Its independent oracle evidence is retained.
  The receiver fallback removal passes all 46 affected tests across six targets. First-class callable,
  invalid-instantiation and failed-constant value deliveries now integrate locally; the combined four-target
  gate passes fifteen tests and retains one strict complete-diagnostic instantiation failure. Constants
  match all five public controls, including real selector failure. Constructor purity and statement
  traversal after termination own the remaining diagnostics. All 80 PHPStan tests pass across 21 current
  executables, with source-matched coverage 10,565/10,565 and no export warnings. Compound-assignment
  completed reads now integrate locally; all four public controls and ten coverage/dead-code neighbors
  pass. The statement walker separates reported unreachable nodes from skipped statements, retaining
  independent declarations and labels. Its first gate removes the six unused-result cascades but exposes
  the distinction between a constructor's never value and nested expression termination; root statement
  value requests now preserve that distinction, with a fresh gate running. Constructor-purity, foreach
  conditional-holder and bare-Closure comparator corrections retain independent source owners.
  The next frozen gate reaches its 900-second coordinator budget after 23 public targets (111 tests),
  466 level cases and 39 rule cases. All completed public and rule targets pass. Its only two level failures
  prove obsolete StringableStrictTypes markers at levels 5 and 9; both are removed, leaving 634 case gaps.
  No new diagnostic mismatch is found in the completed partitions. Interrupted partitions remain unverified.
  All 127 old executables and exact-source profiles are preserved before integrating the three remaining
  deliveries: constant-foreach conditional holders, declaration-owned constructor effects and opaque-Closure
  argument compatibility. Their combined main verification and whole-crate coverage remain open.
  The integrated seven-target main gate now passes all 28 tests, including complete failed-instantiation
  diagnostics, the original foreach regression and asymmetric constructor/callable controls. All four fresh
  PHPStan recaptures match byte for byte. The docs build passes, all six new DOM contracts pass, and the three
  final paragraph captures are inspected. The whole analyser sweep and whole-crate coverage remain open.
  The fresh combined native suite passes all 444 tests. The broader public gate exposes two existing
  callable-branch regressions: opaque-Closure argument acceptance also erases constrained Closure signatures
  during narrowing. Its comparator owner is separating those contracts. The lint gate independently finds
  an incorrect `: null` after a member read on an unknown instantiated class; the native-facts guard must
  recognize missing named-object metadata as well as unresolved references. Original expectations stay
  unchanged. The level sweep proves two more obsolete allowances (ImpossibleInstanceOf bug-13975#1 and
  ParameterCastableToString bug-3946), retained until the frozen sweep finishes. The scratch coordinator now
  proves that its path filters select every fixture case exactly once, without parent-directory overlap.
  Partial source-matched coverage reaches analyse 43,422/43,566 and PHPStan 10,565/10,565 without export
  warnings. This is not a completed whole-analyser gate or full PHPStan parity.
  The frozen sweep now executes all 1,315 rule cases, all 2,787 level cases and all 736 public/library
  tests across 126 targets. Rule partitions pass. Level partitions find no new mismatches and prove four
  obsolete allowances, now removed: ImpossibleInstanceOf bug-13975#1, ParameterCastableToString bug-3946
  and CallStaticMethods bug-8296 at levels 5 and 9. There are 630 level-case gaps and 14 rule-case gaps.
  Public tests pass 734/736, retaining the two constrained-Closure regressions with the comparator owner.
  All 53 targeted CLI tests pass, including all thirteen analyse tests; the earlier 300-second timeout
  was an interrupted check, not an assertion failure. Complete exact-object exports cover analyse
  43,533/43,566 and PHPStan 10,565/10,565 without export warnings. The lint named-object guard restores
  its 28 type-rule tests. Its broader gate finds a native-void read regression in the unchanged PHPUnit
  oracle: the native facts getter now keeps void rather than reading the projected full null value.
  The original assert-equals test passes, and a fresh Rector 2.6.1 recapture matches its unchanged expected
  bytes. The complete lint rerun, comparator integration, retired-gap rerun and remaining coverage stay open.
  None of these unfinished follow-ups is pushed.
  The complete lint rerun now passes all 432 tests across 26 disjoint partitions. Exact build-ID-paired
  exports are warning-free: the sixteen inline-method warnings came from mixing executable maps, not
  stale sources or failed tests. Twenty-five uncovered lint lines are obsolete integer-overflow fallbacks
  after the parser's Float normalization; their source owner is removing them. One real atomic-Never
  assertion control remains. Existing converter and native-artifact exclusions move to their current
  physical lines without broadening their reasons.
  The nullsafe-chain receiver correction passes all twelve targeted groups, including the strict builtin
  probe and genuine null-return control. Original failures remain preserved. Inherited first-class
  callable policies and unavailable-unpack controls now integrate with fresh PHPStan expectations.
  The opaque-Closure subtype correction also integrates locally; it separates assignment acceptance
  from the definite proofs used by intersections and unions. The fresh complete analyser build passes,
  together with all 444 native analyser tests, all 298 codex tests and all five callable-branch controls.
  Source-owned callable restoration controls also integrate: all five array-metadata tests pass against
  69 complete PHPStan records, retaining the original 53 unchanged. Redundant iterable intersections and
  recovered template bounds prove the converter fallbacks remain publicly reachable.
  The private lint cleanup passes 122 tests and covers all 859 owned whole-file lines after existing
  exclusions, with warning-free exact-object exports. Its fresh Rector capture retains throw, never
  and null assertion arguments while changing only the finite control. The combined main follow-up
  passes all 52 affected tests with matching source hashes.
  The final combined gate and coverage now pass as recorded above. The two proven CallMethods
  allowances are retired, leaving 628 level-case gaps and fourteen rule-case gaps. Rule counts are
  unchanged; none of these fixture inventories is a compatibility percentage. CI stays disabled.
- **Rule docs:** steps 1–5 built locally: all 1,256 rule pages, 67 command/group pages, 8 presets, 8 sets, six
  migration mappings and the URL-filtered directory. CLI links need the public docs origin; none is configured.
  The docs build passes on Linux (1,446 pages); public deployment has not been verified.
- **Earlier full batch:** 375 public and library analyser tests pass on Rust 1.99/Linux, together with the unfiltered
  rule harness (1,315 cases, 775 seconds) and all 2,787 level-fixture cases in bounded partitions. The fixture
  checks retain 680 level gaps and 18 rule gaps; they are not full upstream parity. Fresh profiles cover
  41,146/41,146 analyser production lines and 10,406/10,406 owning PHPStan lines after existing exclusions.
  All CLI executables pass, including all 166 Pint rule fixtures (646 seconds), 14 lint tests and 13 analyse
  tests. Coverage cleanup removes stale example executables as well as tests
  and binaries, so old source mappings cannot contribute phantom lines. Harness coverage remains 235/235 lines,
  with all five tests passing in its last full measurement.
- **Local integration gate:** the variable, named-argument and callable snapshot passes the full level-0 rule
  harness (648 seconds), including every variable fixture. Later selection, superglobal and generic-default
  corrections pass their public regressions with unchanged PHPStan expectations. Array-write, coalescing,
  annotation and template-offset corrections are integrated locally. Stricter level checks also enforce the
  new variable and named-argument contracts inside older unrelated gaps. Resolved output-reference receivers
  pass their public controls; the Methods sweep has zero new failures and removes three stale gaps. Incoming
  isset origins, foreach exit types, counterfactual coalescing, error values and unreachable catches pass 62
  combined public controls. Closure/callable output references and named variadic writes pass 21 public tests,
  the Functions rule harness and its strict level sweep. DeadCode, Comparison and Exceptions sweeps pass;
  the two Variables coalesce input-state differences now pass their public controls. All ten exported UnsetRule
  cases pass after nested virtual write-back. Empty-array constructor defaults, never-valued generic arguments
  and nullable template identity fix the readonly ArrayObject offset regression. Allowed-constant argument
  policies pass all three public tests, and the constructor/coalesce/callable controls pass together.
  Mutated shape order, sealed template-array reconstruction, generic unset metadata, mixed subtraction and
  exclusion-aware queries are integrated. All 15 unset tests now pass, including repeated invalid unsets and
  nested parent write-back. The combined reader, array metadata, slicing and mixed controls pass 30 tests;
  the final constant-array reader correction covers 6/6 changed dependency lines without exclusions.
  Input observations restore the sort diagnostic without changing generic defaults or coalesce controls.
  Failed string reads and argument/construct cascade suppression retain primary errors and evaluation.
  The variable level sweep passes, and an earlier full rule harness has zero new mismatches (717 seconds);
  twelve stale level gaps and one stale rule gap are removed. The constant-empty array_rand fallback now
  matches PHPStan's non-never return, fixing the original level-5/9 reachability regression. Its 14 public
  array-rand, slice and dead-code controls pass together. Functions, DeadCode and Variables level sweeps pass
  after removing nine further stale gaps; non-empty count bounds remain isolated work.
  Shared static-closure and unresolvable-parameter contracts are integrated,
  including attribute constructors and first-class callback policies. The call_user_func purity correction
  passes the complete original fixture and 21 callable, attribute and dead-code controls; both targeted level
  filters pass after removing seven further stale gaps. Optional unresolvable-type tips remain isolated work.
  Inferred class-constant receivers and expression guards pass eight public tests at levels 0/1/2/7/8/9/10.
  Incoming explicit-mixed origins survive conditions, while writes invalidate replaced receivers and selectors.
  The Classes rule and level sweeps pass. Call/purity identities, indirect variables, dynamic property selectors
  and general type-dependent comparisons remain unsupported, so the mapping stays inactive. Argument reason
  producers, excluded-mixed/late offset queries and array_rand/random-integer bounds are integrated locally:
  all 262 public analyser tests and the full rule harness pass (483 seconds), and fresh owning PHPStan coverage
  is 10,068/10,068. All 628 changed production lines are covered. Functions, Methods, Arrays and DeadCode level
  sweeps pass. Broader level and whole-analyser coverage gates are running; Comparison and Properties found
  a count-after-loop-unset error and an extra generic-property accessory. Both reproduce on pushed main before
  this batch. Their source-owned corrections are now integrated: definite-key extraction requires every offset
  alternative to be literal, and bounded constant-list foreach replay retains sequential array exit states.
  Eight obsolete gap entries are removed. Literal-template scalar buckets, exact Zend union sorting, attribute
  input corrections and named-call class-constant guards are integrated locally. The combined template/attribute
  public matrix and the complete PHPStan crate pass. Owning-crate coverage exposed the object-template union's
  strategy tie-break; 96 fresh PHPStan controls now pass after matching inherited SubtractableType dispatch.
  Every public analyser target passes across a split instrumented run, and all 12 analyse CLI tests pass.
  Trait constructor reflection is now integrated, preserving adapted visibility, storage identity, consuming-class
  parameter scopes and effective return types. That snapshot and the array-mutation corrections pass the full
  batch above, with eight obsolete level gaps removed. Negative mixed guards match all 92 strict oracle records
  in isolation, including declared versus inferred assertions and subtraction descriptions. Their current-base
  replay also preserves explicit inferred mixed metadata: 105 public tests pass and 114/114 changed production
  lines are covered. Parser validation, imported compact names, source-only alias rejection, canonical global
  initializers and required native unset reflection are now integrated locally. Their 87 combined library and
  public tests pass. Genuine checked StrictMixed copies now preserve source-owned explicitness, template
  identity and defaults without changing ordinary inference. All 66 PHPStan tests, 53 strict-mixed and unset
  controls, and the four-level mixed-property regression pass on main. Ten stale coverage exceptions are removed.
  Native annotation writable-type selection is also integrated; all 27 unset tests and 60 combined library and
  public controls pass. All 13 analyse CLI tests pass. Member-modifier parser validation is integrated; all seven
  parser tests pass and both original property fixtures match, removing two rule gaps. Parameter parser checks
  are also integrated; all 11 parser tests pass. Class-modifier checks are integrated from the isolated 14-test
  parser gate, with 525/525 parser lines covered there. Fresh combined gates are running.
  All level-fixture groups pass the preceding integration snapshot, including all 12 Methods classes after
  splitting the 900-second aggregate timeout. Both Properties
  rule and level sweeps pass the latest native-writable/member-parser snapshot. Filtered gates now retire
  gaps only for executed cases; the 33 previously misreported skipped cases remain in the gap list. One real
  stale Methods gap is removed after its complete fixture passes. The focused filter regressions pass, and
  unfiltered gates still reject orphaned gap entries, verified against controlled fixture roots before/after.
  The combined rule sweeps cover all 1,315 cases with 18 retained gaps; Functions and Methods now pass the
  parameter-parser snapshot (380 and 366 seconds).
  The strict-mixed integration also passes constant-initializer, generic-default, coverage and native-lane
  controls. The PHP 8.0 readonly parser-admission correction is now integrated, matching the three original
  diagnostics without changing expectations or gap allowances. Its isolated parser and statements suites pass
  16/16 and 18/18, with 602/602 parser lines covered; both suites also pass on main. The latest Properties
  level sweep passes after removing one verified stale gap, leaving 696 level gaps and 18 rule gaps.
  Full combined fixture gates and fresh owning-crate coverage remain before delivery.
  Local phar:// archive checks and PHP's two-layer gzip restart limit are integrated: all seven include tests
  and 43 combined argument, array and unset controls pass. Fresh archive expectations match the real PHPStan
  public Analyser byte for byte. Runtime wrappers, PHAR aliases/mounts, OpenSSL signatures and bzip2 remain
  unsupported. The latest full rule harness found one open-shape unpack-count regression; the upstream
  non-empty fallback fixes it, with the complete CallMethodsRuleTest passing and 5/5 changed lines covered.
  New rules remain inactive. The broad public run passes 245 tests; its sole failed legacy Mago
  comparator now passes with the reader/unset controls after accounting for PHPStan's cascade policy using a
  fresh oracle capture. Its original Mago capture is unchanged. The combined PHPStan crate passes all 54 tests;
  fresh owning-crate coverage is 9,735/9,735 after the public class-name compound-query controls.
  Full combined analyser fixture gates and coverage still need measurement. The 1,446-page docs build and
  rendered call, unpacking and archive-contract paragraphs pass inspection.
- **Current batch:** lint batch 1 is integrated locally: 325/703 lint rules, with config, fixer and all 243 lint
  tests passing on the merged snapshot. Generic callable specialization, class variance, compact extraction,
  class-initializer unions, passive native branches and accepted try/hook/nested-namespace validation are also
  integrated. The generic correction passes 50 fresh PHPStan comparison records and 73 combined public/type
  tests. All 363 analyser public/library tests pass after trait-context integration. Two direct-executable
  checks initially used the wrong working directory; both pass from the owning crate, matching Cargo.
  The NaN default-parameter regression now passes unchanged after matching PHPStan's constant-float identity;
  all 69 owning-crate tests and six fresh
  scalar/shape oracle controls pass. Every one of the 2,787 level cases and 1,315 rule cases passes in bounded
  partitions, with 680 level gaps and 18 rule gaps retained. The unfiltered rule run exceeds 900 seconds; the
  partitions execute every case, not an unfiltered success. Sixteen stale level gaps are removed without changing
  expectations. Fresh coverage is 10,406/10,406 PHPStan lines and 40,972/40,972 analyser lines after existing
  exclusions. Trait initializer caches preserve implementing-class selection across inheritance and explicit
  trait reuse; consuming declaration checks remain separate. Whole-union conflicts, keyed source order and
  reflected magic contexts match six fresh captures with 143 diagnostics and an independent ten-dump control.
  All five public trait tests and scoped Classes, Constants, Traits, Arrays and Properties gates pass on main.
  A fresh negative control confirms unsupported call initializers do not create false trait-value conflicts.
  Dynamic holders, calls/new precision, expanded cycles and general mixed/spread array order remain incomplete.
  Coverage also found a static-receiver bug: class-string object bounds must be unwrapped before method-capability
  checks. Fresh ordinary-call and first-class-callable controls fail before the correction and pass afterward,
  including stdClass and unchecked mixed bounds. Its 26 public neighbors, complete Methods rule sweep and static
  method level sweep pass. All 13 analyse CLI tests also pass after the receiver correction.
  The CLI rule fixture loop now uses bounded Rayon parallelism:
  all 166 fixtures pass in 776 seconds with two threads, preserving every assertion and expectation.
  All other CLI executables pass, including all 13 analyse machine-format tests. The Pinkary format comparison
  matches all 799 files; its absent-dependency analysis errors are not analyse/check parity evidence.
  All 20 format CLI-parity argument sets match. Mutation parity matches Pint on 272 Pinkary app/test files
  (seed 7) and 117 Laravel Support files with forced CRLF (seed 3), using the fresh batch binary.
  The 1,446-page docs build and rendered contract paragraphs pass inspection. The combined coverage gate is
  closed. No new analyse rules or CI workflows were activated.
- **Latest integration:** attributed global constants, namespace postprocessing, leading-shebang token admission
  and compilation-unit entry brace rejection pass the full batch above. Four fresh PHPStan public captures are
  byte-identical to their expectations. Lint batch 2 also passes: Composer class lookup, native
  type corrections and the affected oracle fixtures follow Rector with PHPStan 2.2.7, matching both Pinkary
  and the corpus gate. Both Rector oracle locks now pin that version; the analyse oracle stays at 2.3.1.
  Documented constant return types and anonymous-class finality match a fresh Rector capture without changing
  expectations. All 249 lint tests and four Prettier worker tests pass. Fresh Pinkary lint comparison: 33 agree,
  zero Rector-only and zero Pun-only. The framework Rector capture completed with one existing malformed fixture.
  Both instrumented and native-debug Pun runs timed out at 900 seconds without JSON; bounded positional-file
  splits preserve the full configured ProjectCodebase. The lint-batch-2 snapshot covers 3,024/3,027 files:
  6,304 agreeing pairs, 75 Rector-only and 93 Pun-only. This is a partial comparison, not current-main parity.
  Eloquent/Builder.php, Http/Client/Batch.php and Validation/ValidationRuleParser.php each exceed 300 seconds.
  The full source digest remains unchanged. A nine-line Builder-shaped condition reproduces redundant object
  intersections growing inside mixed exclusions; a source-owned correction is assigned in isolation.
  Existing fixtures cover implicit magic debug returns and overloaded native overrides, with byte-identical fresh PHPStan recaptures.
  Combined coverage is closed without new exclusions. No mappings or CI workflows were enabled.
- **Initializer and parser batch:** canonical unsupported-initializer fallback independently matches all 19
  PHPStan diagnostics. Its snapshot with PHP 8.5 pipe-parentheses admission passes 380 public/library tests
  and the complete 1,315-case rule harness (868 seconds), retaining the same 18 gaps. Reference-to-new rejection
  also passes 77 combined public controls and three freshly rebuilt constant-override tests. Current main passes
  Classes, Constants, Traits and Operators level sweeps and five assignment/instantiation/pipe rule gates.
  Fresh owning coverage is 41,201/41,201 analyser lines without new exclusions. The 1,446-page docs build and
  rendered fallback, pipe and reference contracts pass inspection. Raw `new` identities and for/echo comma checks
  are now integrated with empty-array admission: 397 public/library analyser tests and all 249 lint tests pass.
  The complete 1,315-case rule harness passes in 829 seconds; Classes, Constants, Traits, Arrays and Operators
  level sweeps pass with the same 680 level gaps and 18 rule gaps. Fresh coverage is 41,291/41,291 analyser
  lines, including 938/938 parser lines, without new exclusions. Fresh oracle recaptures match the object,
  reader, comma and empty-array captures. The 1,446-page docs build and final rendered contracts pass.
  An additional relative-object member probe exposes eight missing diagnostics for raw self/parent/static
  receivers; failed invocation types and ordinary method unknown-class reporting remain a separate source-owned
  follow-up. Finite conditional holders with ordered canonical joins are integrated locally.
  Destructuring target roles are also integrated: 127 public tests, 69 affected rule cases and 146 Variables/Arrays
  level cases pass. Fresh assignment expectations are byte-identical to PHPStan. The unfiltered rule run timed
  out at 900 seconds; only the affected fixture groups are counted as passing for this subsequent correction.
  Fresh owning coverage is 41,302/41,302 analyser lines, including 505/505 assignment lines and all 47 changed
  executable lines, without new exclusions. The docs build and rendered assignment contract pass inspection.
  Finite holders and literal-array read observations now pass all 409 public/library analyser tests, all 249
  lint tests and all 13 analyse CLI tests. The fresh normal binaries build. Independent cold PHPStan recaptures
  match all 99 holder diagnostics and the complete seven-level assignment-read capture. The 1,446-page docs
  build and both rendered contract paragraphs pass. Complete bounded rule partitions execute all 1,315 cases;
  Variables, Arrays, Constants, Classes and Traits level sweeps pass with the same documented gaps. Fresh owning
  coverage is 41,434/41,434 analyser lines. All 370 changed executable lines, including the maintained Codex and
  Analyzer dependencies, are covered without new exclusions. No mappings or CI workflows changed.
  Mixed document closing-marker rejection and failed named method results pass 418 public/library tests,
  all 1,315 rule cases, 1,508 Functions/Methods/Operators level cases, all 249 lint tests and all 13 analyse
  CLI tests. Cold PHPStan recaptures match all 81 method records and the complete closing-marker capture.
  The active PHP-worker/Rust-provider control passes. Fresh owning coverage is 41,448/41,448 lines, with all
  32 changed executable lines covered across Pun and the maintained analyzer, without new exclusions.
  Fresh normal binaries and the 1,446-page docs build pass; both rendered contract paragraphs pass inspection.
  The same 680 level gaps and 18 rule gaps remain. No mappings or CI workflows changed.
- **Document bodies and named functions:** 428 public/library analyser tests, all 1,315 rule cases and all
  2,787 level cases pass in bounded partitions. Both 220-case CallMethods replacements pass after the combined
  partition reached 900 seconds. All 249 lint tests, 13 analyse CLI tests and the PHP-worker/Rust-provider
  control pass. Fresh PHPStan captures match all 56 named-function records and the complete document-body
  capture. Dynamic-call match conditions retain their primary callable error without spurious comparisons.
  Fresh profiles cover 41,656/41,656 analyser lines, including 1,155/1,155 parser lines and all 212 changed
  executable lines across Pun and the maintained analyzer, without new exclusions or earlier profiles.
  Real CLI JSON and exit codes match PHPStan for severe parser failures and failed-function argument writes.
  Normal binaries and the 1,446-page docs build pass; both rendered paragraphs pass inspection. The same
  680 level gaps and 18 rule gaps remain. No mappings or CI workflows changed.
- **Numeric, purity and string batch:** malformed legacy-octal admission, conservative `void.pure`, precise
  `strlen()` returns, lint batch 3 and embedded PHPStan API PHPDoc imports pass 438 public/library analyser tests,
  all 253 lint tests, all 1,315 rule cases and all 2,787 level cases in bounded partitions. All 28 affected CLI
  tests pass in bounded runs, including the four purity configuration controls. The API correction retires one
  rule gap, leaving 680 level gaps and 17 rule gaps. Full purity mappings stay inactive: 24 dedicated upstream
  diagnostics are strictly supported, while 51 level-fixture locations remain explicit diagnostic allowances.
  The broad coverage-fixture run first failed at six additional unsupported `void.pure` locations; recording
  those exact locations makes its complete three-test rerun pass without changing oracle expectations.
  Fresh profiles cover 42,173/42,173 analyser lines, 22,064/22,064 lint lines and all 663 changed executable lines
  across Pun and maintained syntax, without new exclusions. Normal binaries, the provider control and the
  1,446-page docs build pass. All four rendered contract paragraphs pass DOM checks. Pinkary format matches
  all 799 files; absent-dependency analyse errors are not analyse/check parity evidence. The fixture run still
  catches an empty-union panic in native intersection scanning; its backtrace is retained for the follow-up.
  No mappings or CI workflows changed.
- **Flat PHPDoc unions:** iterative lowering removes the normal-stack 129-arm overflow while retaining every
  binary normalization and nullable boundary. All 119 combined public tests, four Types/PhpDoc rule cases and
  148 Generics/PhpDoc level cases pass with unchanged gap contracts. A fresh PHPStan recapture matches all
  38 records; fresh profiles cover 40/40 changed engine lines and 33/33 test lines without exclusions.
  The 1,446-page docs build and both rendered contract sentences pass. Parsing and nested non-union types
  retain their existing stack limits; no stack-size or provider-budget changes were made.
- **Native scalar casts:** the subsequent default-off slice passes 37 combined native, callback, slice, debug
  and arithmetic tests, including 133 actual PHPStan native dumps. Its changed engine lines are 152/152 covered.
  It models 64-bit PHP with default `precision=14`; unsupported transfers stay unavailable and native-dependent
  rules remain unported. This slice changes no Pun production lines.
- **Numeric constants:** full inference retains non-finite arithmetic, PHP float-modulo semantics, platform
  unions and evaluated declaration values across function-local artifacts. The new public fixture matches all
  43 PHPStan diagnostics; the full batch above includes it and a byte-exact literal-key regression. All 12
  analyse CLI tests pass. Forward/scanned initializer resolution is integrated locally against 29 further
  PHPStan diagnostics; combined verification is running.
- **CLI coverage:** fresh full-registry measurement is 5,457/5,457 lines, with all executables passing. Non-ignorable
  errors retain their rule's flag in reports and cannot be hidden by ignore comments or baselines.
- **Lint coverage:** the Pest/Mago integration snapshot measures 29,413/29,413 Linux full-registry lines,
  with 352 lint tests passing. Fresh source-matched LLVM data restores the facts.rs:684 upstream-analyzer
  failure exclusion already present on origin/main and removed during the Pest coverage pass.
  No deterministic public failure input is known.
- **Fixer coverage:** Linux full-registry measurement is 36,563/36,563 lines, with 106 tests passing on main.
- **Laravel alias groundwork:** installed defaults, literal app configuration, config/package caches and Composer
  discovery feed shared alias metadata without PHP. Pinkary level 0 matches all eight oracle diagnostics for
  ported identifiers; dynamic bootstrap and runtime lookup order remain unported.
- **Framework level 0:** 378/378 oracle diagnostics match by file, line, identifier and message for ported
  identifiers. This does not cover unported rules or higher levels.
- **Closure bindings:** inline `@param-closure-this` callbacks use single-class typed and nullable receivers at
  level 0, without enabling level-2 method checks. Multi-class receiver bindings remain unported.
- **Integrated inference batch:** production type dumps, source array/template metadata, raw callback gathering,
  array slicing, required-parent receiver identities, first-class callables, pipes, invalid throws and constant
  covariance pass their oracle regressions. Config's 54 tests, all 12 analyse CLI tests and the 1,446-page docs
  build pass. Sealed unpacking and inferred duplicate keys remove one Arrays gap, but float and constant-key
  boundaries still block whole-rule activation. Include checks and restored upstream targets pass. A default-off
  partial native lane is integrated; native-dependent debug rules stay unported. Laravel callback regressions
  match the real Larastan oracle after selective observer filtering and property-cache synchronization, with
  27/27 changed production lines covered separately, including the maintained analyzer dependency. Pinkary's
  canonicalized corpus harness matches all eight level-0 oracle diagnostics. Attribute, dynamic-instantiation,
  scalar-constant, finite dynamic class-constant, native-operator and arithmetic corrections pass combined
  verification. Three resolved class-fixture gaps are removed. Attribute constructor types and references match
  the level-1 and level-5 oracle controls. Shared `@no-named-arguments` handling passes 28 integrated public
  controls; attribute mappings await the combined gate. The docs build passes with the updated call contract.
- **Mago 1.59.0 review:** upstream still needs Pun's raw callback, observer-isolation, passive-native and integer
  overflow patches. Keep all crates at `=1.57.0` while backporting the isolated `Closure::fromCallable()` provider
  and omitted-template defaults against PHPStan's oracle. The newer boolean and `elseif` fixes inform the
  variable work but do not prove correlated-absence parity. A full upgrade must forward-port every maintained
  patch and move all pins together.
- **Earlier parked work:** `wt-lint-rules2`, `wt-lint-types` and `wt-check` are absent from this checkout and origin's
  current refs. Inspect history before recreating any of their work.
- **PHPStan parity metric:** context/reference/phpstan-identifiers.txt: the 947 identifiers phpstan.org/error-identifiers lists for phpstan/phpstan-src 2.3.x (latest release 2.3.1); `pun_analyse::reports()` recognizes 690 (2026-10-26), checked against the freshly compiled public API. This is identifier coverage, not full behavioral parity. Extensions are tracked apart: phpstan-strict-rules-identifiers.txt (36 of 53) and phpstan-deprecation-rules-identifiers.txt (72 of 120). The oracle is PHPStan 2.3.1 (wt-oracle-23).
- **Pest batch 4:** all 13 delegated commits are integrated locally, providing 62 native Pest rules and 58 newly
  ported mapping entries. Mapping validation has zero errors and all 1,424 generated files are in sync under
  Python 3.13. Combined gates pass 352 lint tests, 56 config tests and all 41 affected CLI tests. Fresh Pinkary
  comparison matches all 33 file/rule pairs with no residuals; fresh lint coverage is closed above. Delivery
  is covered by the corrected combined gate below. Documented exceptions remain
  fixpoint rewrites, whole-file inline-HTML reprinting, rejected invalid Rector output, and the untracked
  `tests/Pest.php` cache dependency. The lint mixed filter is unchanged.
- **Mago batch 5:** the three functional/doc/test commits are integrated locally, adding 33 native rules.
  The direct `mago-phpdoc-syntax = "=1.57.0"` and `regex = "1"` dependencies reuse existing locked packages.
  The delegated coverage-only commit is omitted: its facts.rs exclusion refers to a different source revision,
  and the fresh combined-main result replaces its figures. Mago rule-level options defer to
  pun.toml levels; the optional Drupal function-name convention remains unsupported.
  Mail transfer stripped CRLF from two upstream byte-offset fixtures; their exact mailbox bytes are restored
  and protected from checkout newline conversion. All 34 Mago-batch tests now pass with unchanged expectations.
  The 1,446-page docs build and rendered callback, explicit-mixed, filter and option-boundary paragraphs pass.
- **Lint batch 6 provenance:** both mailboxes are present and their sizes and SHA-256 digests match delivery:
  297,168 bytes for the five dead-method rules, and 5,479 bytes for the mixed-receiver follow-up.
  Fresh independent Rector research confirms its ordinary-call mixed normalization; do not add an explicit-mixed
  exclusion to the fluent fallback. The follow-up fixes the stale explanation and pins six asymmetric receiver
  controls to actual Rector output. Both mailboxes are now integrated locally with the PHPUnit replay below;
  their combined-main gate remains open.
  The reported facade-fix/private-method deletion is verified on the older combined tree d025d6ea, not
  reproduced on current main. Fresh serial checks of all three reported Pinkary files match Rector 2.6.1.
  Isolated IncrementViews runs match Rector with default concurrency and one process, for both the canonical
  input and the older whitespace-only variant. Receiver types remain present after the facade fix.
  The failed combined tree's source object is unavailable here, so its precise cause remains unresolved;
  missing post-fix analysis is not established. A test-only public Linter::fix guard now checks called and
  unused private methods together with nested facade rewrites. Its private six-test owner suite and facade
  control also pass on combined main. No production correction is applied. The replayed rule
  is already on origin/main through 6817472e; do not reapply the old mailboxes.
- **Explicit-mixed and never batch:** the initial baseline passed 443 public/library analyser tests,
  253 preceding lint tests, 28 CLI tests and all 1,315 rule cases. Its 2,787-case level sweep exposed four genuine
  Functions regressions and twelve obsolete gaps. The filter repair is now integrated: fresh PHPStan recaptures
  match 111 observations and four level-9 diagnostics. The callback repair passes 33 neighboring public tests,
  including the original fixture and fifteen fresh PHPStan observations, with 32/32 changed engine lines covered.
  Nested foreach keyed writes are integrated independently of the queued object-intersection changes: all 55
  affected public tests pass, and all 35 fresh oracle diagnostics match after canonicalizing worktree paths.
  The original bug-13643 fixture now reports zero errors, as PHPStan does. The twelve obsolete gaps are removed;
  corrected Methods levels 5/9, static-method level 9 and increment/decrement level 9 partitions pass. The
  first Functions level-9 rerun exposed one failure: implicit `never` from invalid filter input incorrectly
  terminates the call and suppresses unused-result/next-statement diagnostics. PHPStan preserves that return
  observation but derives termination from the resolved acceptor's explicit declaration, not the dynamic
  extension. The source-owned repair now separates call facts from value types, including speculative
  checkpoints and nested evaluation. The never-kind prerequisite is integrated. The fresh 70-diagnostic
  PHPStan capture also checks forwarded implicit values and PHPDoc never over object, array and void native
  declarations. Residual template extraction now retains bottom identity. All 70 public records and the
  complete 246-case Functions level-9 fixture pass without new gaps. The neighboring public controls pass,
  except four coalesce messages that also fail on the retained pre-repair executable. Their source-owned
  required-offset correction is now integrated, with all 20 fresh PHPStan records matching on main.
  Public replay controls reproduce stale terminating facts after hook skips and unresolved targets;
  expression-entry defaults fix all twelve condition/increment, hook and native-tracking combinations.
  All 81 combined public tests pass across twelve suites. The broad sweep also verifies four newly passing
  coalesce, nullsafe and random-integer gap entries, which are removed. The pre-repair broad run finishes
  all 1,315 rule and 2,787 level cases with only those stale gaps and the two repaired coalesce suites failing.
  The template-offset control also passes on corrected main. Fresh combined fixture, public, CLI and coverage
  verification passes, including the token suite. All 799 Pinkary format files and 20 CLI argument sets
  match Pint; mutation runs match 272 Pinkary files and 117 Laravel Support files with forced CRLF.
  Do not replace never with mixed or hide the mismatch. The 1,446-page docs build and rendered never,
  coalesce and option-boundary paragraphs pass.
  The corrected gate passes all 328 jobs: 1,315 rule and 2,787 level cases, 468 public/library analyser tests,
  352 lint tests, 69 PHPStan tests, 33 token tests and all 14 affected CLI tests. The obsolete intersection retry
  is removed, and all 19 native-lane controls pass, including observer-free convergence and special returns.
  Fresh source-matched coverage is 42,527/42,527 analyser lines, 29,413/29,413 lint lines,
  10,406/10,406 PHPStan lines and 3,901/3,901 token lines after existing exclusions. Token coverage combines
  the full system-PCRE2 10.42 suite with the pinned supporting-engine controls on static PCRE2 10.46;
  the sources are byte-identical. All 8,117 changed production lines since the preceding pushed main are covered.
  The 16 zero-hash LLVM warnings name unchanged inline flag methods, each with positive matching profile data.
  An existing callable exclusion is removed, the symbolic-key exclusion moves with its unchanged branch,
  and the lint facts exclusion already present on origin/main is restored. No new exclusions are introduced.
  The combined gate is closed; 664 level gaps and 17 rule gaps remain. CI stays disabled.
- **Native intersections:** the empty-intersection scanner correction is integrated. Seven freshly
  recaptured signatures match all 14 PHPStan parameter/return types and five public diagnostics. The isolated
  gate passes 96 focused tests with 3/3 changed scanner lines covered; all 36 combined-main native, callback,
  filter and explicit-mixed controls pass. Removing the obsolete masked-source retry also passes 101 combined
  parser, initializer and native-intersection controls on main.
- **Inference batch verified:** current-file closure and anonymous-class metadata, clone results, receiver-aware
  native properties, nonaccepting exit/die/throw, simultaneous ASCII casing, lossless constant-string bytes and
  str_repeat inference pass the combined gate. All 1,315 rule cases, 2,787 level cases, 503 analyser tests,
  353 lint tests, 70 PHPStan tests, 33 token tests and 28 affected CLI tests pass. The original 350-job run found
  an anonymous-class finality regression in lint's refresh scan. Its correction passes a fresh 37-job complete
  lint and affected CLI/analyser rerun. The initial failure is retained in the evidence, not overwritten.
  Source-matched profiles cover analyse 42,624/42,624, lint 29,444/29,444, PHPStan 10,414/10,414 and tokens
  3,956/3,956 lines after existing exclusions. All 1,025 changed executable source lines are covered without
  new exclusions. Nineteen standalone protocol controls pass with protocol 2.1. Three fresh main PHPStan
  captures match byte for byte. Pinkary format is identical on 799/799 files; all 20 format CLI parity sets
  and mutation checks on 272 Pinkary and 117 framework files pass. The docs build passes on 1,446 pages,
  including six rendered contract paragraphs. The existing 664 level gaps and 17 rule gaps remain unchanged.
- **Queued batches:** object-intersection normalization is integrated locally. Its 351-job gate finished with four
  regressions: fluent receivers, exact instantiated-class alternatives, short-circuit output certainty and
  initializer null removal. Existing expectations and gap allowances are unchanged. The initializer repair
  follows PHPStan's explicit-null-only removal; its fresh capture matches all 19 existing diagnostic records.
  The boolean-certainty repair is integrated from its isolated 51-test gate and 40/40 changed-line coverage.
  Receiver identity and exact-instantiation repairs are integrated; 57 combined-main object, native and string
  controls pass. Exactness remains absent from Pun's separate converted PHPStan object types, so debug-type
  conversion can still collapse exact alternatives. The concatenation repair matches
  all 29 fresh PHPStan observations, including the partly-nonempty and repeat controls. String-union retention
  now matches all 52 further observations after preserving implied versus independent nonempty accessories.
  The dedicated numeric capture matches all 17 records; existing expectations remain unchanged.
  Nested variable-name reads and multiline name-error locations pass all four oracle configurations and
  50 combined-main public controls. All 164 earlier variable records remain unchanged; 8/8 changed production
  lines are covered. The dead-method callback refactor passes its five public tests and 1,327/1,327 owned
  lines in isolation without exclusions. A fresh combined fixture, CLI and whole-crate coverage gate is running.
  The controlled performance probe and all three previously stalled framework files
  now finish. The five-rule dead-method and 93-rule PHPUnit batches are integrated locally from the verified
  isolated replay and still need combined-main verification. The current-main fixer fixture target passes;
  the independent replay counts 15,474 cases. All 1,424 generated docs files are in sync, the 1,446-page build
  passes, and browser DOM checks confirm the 981-port directory count and new dead-method/PHPUnit statuses.
  The lint count is 514/703, not 514/699: the PHPUnit report omitted four unported mappings from its denominator,
  not from the catalogue.
  The repaired snapshot matches all 799 Pinkary format files and all 20 format CLI argument sets. Fresh mutation
  runs match 272 Pinkary files and 117 framework files with forced CRLF. The 553-job gate executed all 1,315 rule
  and 2,787 level cases without new fixture mismatches. It found a native object-union declaration-order
  regression in two named-argument public tests. Removing constructor sorting preserves written receiver
  order while leaving equality, hashing and printed IDs order-independent; the isolated public suite passes
  15/15. This correction is now integrated. Four executed, obsolete Methods gap markers are removed, leaving
  660 level gaps and 17 rule gaps; their full groups are running again. The complete fixer fixture retry passes
  all 15,474 observations after the matrix's 900-second timeout. The all-rule CLI test also exceeded 900 seconds
  twice; an unchanged-executable retry passes all 166 cases with four Rayon threads in 735 seconds. Original failures and supplemental results are
  retained separately. Fresh raw coverage reaches all 13,477 changed executable production lines, but whole-crate
  coverage still misses four truthy-initializer lines and four fluent-return lines. Those branches are isolated
  work, not new exclusions. Reports apply cargo-llvm-cov's normal test-file filter; the existing facts exclusion
  moves with its unchanged closing-brace region.
  The fluent-return branch is reachable: one fresh Rector fixture returning `$this` and `new self()` requires
  `self`, covers all four missing lines, and passes all 28 public type-rule tests privately. Its fixture-only
  commit is integrated after the preceding frozen gate's coverage export. A second integrated fixture-only
  commit covers all four truthy-initializer lines and passes 33 public neighbors. Removing materialization makes its
  nonempty-array assertion fail; restoring the unchanged production block makes it pass.
  Parsed compiler-halt detection is integrated: it matches 20 fresh PHPStan cases and covers 19/19 changed lines.
  Stable array-offset name filtering matches 236 four-mode records and covers 75/75 changed lines; literal
  string-byte offsets extend that capture to 318 records, with all 26 affected public tests passing and 15/15
  further changed lines covered. Both are integrated and pass the complete combined analyse/lint gate.
  The docs build passes all 1,446 pages; rendered DOM checks confirm the six updated contract paragraphs.
  All 109 analyser/lint targets compile after repairing one existing unit-test Checker initializer. The new
  normal binaries build and match all 20 Pint CLI argument sets. All 13 analyse CLI tests pass in bounded
  runs: twelve tests together and the machine-format test separately. The original combined compile/test
  command exceeded 900 seconds; that timeout remains explicit evidence.
  The new combined gate found one real regression in level-3 Methods/ReturnTypeRuleTest::testBug12984:
  an extra ArrayIterator return-type error whose expected and actual descriptions print identically.
  Positional compound equality rejects differently ordered alternatives inside invariant generic arguments.
  A private source-owned repair matches PHPStan's one-use member comparison without sorting stored types;
  all 71 PHPStan tests, 21 object-intersection tests and 15 named-argument controls pass, with 7/7 changed
  production lines covered. The repair is integrated, and all 71 owning PHPStan tests also pass freshly on main.
  The original PHP fixture and empty upstream expectation stay unchanged; the complete Methods level sweep
  passes inside the fresh combined gate.
  The complete lint type target exposes two declaration-order regressions after shared union sorting was
  removed: an arrow return writes `array|int|bool` instead of `array|bool|int`, and a closure writes `string|float`
  instead of `float|string`. A PHPUnit callback writes `string|int` instead of `int|string` through the same
  declaration helper. The lint inference/declaration correction is integrated from its passing private gate;
  upstream fixtures and receiver-order controls stay unchanged. The fresh 372-job combined gate passes
  against the repaired sources, including all 1,315 rule cases and 2,787 level cases. Five verified obsolete
  gap markers are removed, leaving 659 level gaps and 17 rule gaps. Coverage exports verify unchanged inputs
  across the entire run; source-matched whole-crate and changed-line coverage are closed.
  General integer/range string offsets pass 340 variable records and 78 dump records privately, with 55/55
  changed lines covered. Cached failed reads before coalescing remain an isolated follow-up.
  Static-property inc/dec checks the stored new value independently of postfix's old expression value.
  All 23 statement tests and 431 supported oracle records pass on the combined snapshot. The original upstream
  fixture is unchanged; two existing unsupported protected-set diagnostics per level remain in its raw captures.
  Properties sweeps have no new mismatches. The complete combined gate verifies removal of one obsolete rule
  gap and three level gaps. Resolved loop settings match all 292 fresh oracle records, and normal/late PHPDoc
  shape keys retain decoded bytes. Per-value object exactness still needs to survive conversion into Pun's
  PHPStan types; that source-owned correction is isolated. Rule activation, the worker protocol and
  declaration-level finality stay unchanged.
  The fresh Pinkary lint comparison selects all 306 ported Rector classes. Its parallel Rector run omitted
  changes in twelve test files despite reporting no errors. A single-file control and a complete sequential
  `--debug --clear-cache` rerun recover them: all 33 file/rule pairs agree, with zero residuals. Applying both
  tools' JSON diffs with zero fuzz produces byte-identical output for all 29 changed files, before and after
  pinned Pint normalization. All 342 original source digests remain unchanged. The original parallel result
  remains in the evidence; it is not evidence of a rule bug. The reason for its incomplete output is unresolved.
- **Priorities, in order:** (1) done: restore the combined level, rule and coverage gate after call-flow integration, retaining documented older gaps; (2) done (lint vs Rector: `scripts/lint-vs-rector.py`, residuals in §3); (3) done (lint declaration-order regressions and source-matched coverage); (4) doing: finish analyse level 0 after the exact nonexistent-parent batch, the
  Larastan groundwork and levels 1-10; (5) lint rules by usage; (6) `pun check` in one pass; (7) perf rounds with
  `scripts/bench.py`.
- **Waiting on Pushpak:** audit 26 (agent-mode `OnceLock`), `style/prefer-class-constant`, independent batch-8
  integration, CI re-enable. Batch 8 remains queued while the approved work continues.


## 1. This folder

| Path | What it is | Edit when |
| --- | --- | --- |
| PLAN.md (this file) | Decisions (§2), build order and status (§3), open questions (§4) | A decision, a step's status or the order changes |
| [plan/](plan/) | One plan per part, each with Today, the design, a **Build** list with "done when", and Decisions: [commands](plan/commands.md) (who does what, `pun check`), [config](plan/config.md) (pun.toml, names, presets), [rules](plan/rules.md) (placement, overlaps, naming, ignore comments, mapping data), [output](plan/output.md), [format](plan/format.md), [lint](plan/lint.md), [analyse](plan/analyse.md), [migrate](plan/migrate.md), [init](plan/init.md), [docs](plan/docs.md) (rule pages, rules directory, migration mapping), [foundations](plan/foundations.md) (CI, harness, docs, release, later) | We discuss or change that part |
| [reference/](reference/) | [oracle.md](reference/oracle.md) (pinned upstream tools, exporters), [tokens.md](reference/tokens.md) (pun-tokens API), [mago-extras.md](reference/mago-extras.md) (Mago-derived rules) | The thing it describes changes |
| [AUDIT.md](AUDIT.md) | Code audit findings, P0 to P3, from `prompts/rust-audit.md` | An item lands (tick it) or a new audit runs |
| [prompts/](prompts/) | [rust-audit.md](prompts/rust-audit.md): prompt for an agent to audit the Rust code for smells and unidiomatic Rust; writes `context/AUDIT.md` | Pun's conventions or lint policy change |
| [reports/](reports/) | [scoreboard.md](reports/scoreboard.md) (parity, fuzz, benchmarks; generated by `pun-harness`), [COVERAGE.md](reports/COVERAGE.md) (generated by `scripts/coverage.sh`) | Regenerated; notes by hand |

Rules of thumb:
- One fact lives in one place. A decision is a line in §2; the detail and the reasons live in the plan file it links.
- When a decision changes, edit its line and date it. Never add a second line that contradicts it.
- Before starting a row in §3, set its Status to `doing`. When it lands, delete the row; don't keep a "done" list.
- Keep §0 current: counts, what is red, the next priorities.

## 2. Decisions

All from Pushpak unless marked "assistant's call"; dated 2026-10-17 unless given.

### Product

- **Home and licence:** github.com/punphp/pun, MIT (LICENSE, THIRD_PARTY_NOTICES.md). Not a shipfastlabs project. The
  repo stays private for now.
- **Positioning:** Pun is to PHP what Bun is to JavaScript. Laravel first, PHP 8.4+ projects only.
- **1.0 ships everything at once.** Nothing is published or tagged until `pun format`, `pun analyse`, `pun lint` and
  `pun migrate` all exist and meet the 1.0 bar below. The site presents that release as 1.0. Publishing needs Pushpak's go.
- **The 1.0 bar:** every lint rule (all 703) native; every PHPStan core rule (327 Rule classes) native; Larastan built in with seven
  improvements (typed config, routes, views, translations, env keys, Inertia pages, fresh model data); deprecation-rules and strict-rules as plain rules; a result cache. Speed and memory
  come first in every command.
- **Brand:** "Sticker Pop", the sprinting elephant in sneakers; Fredoka for text, Martian Mono for code (design/).

### Commands (plan/commands.md)

- **One job each.** `pun format`: how the code looks (layout, casing, spelling; never changes behaviour, never deletes
  code). `pun lint [--fix]`: a better way to write it (Rector, PHP-CS-Fixer's risky fixers, Mago's lint rules, version
  upgrades). `pun analyse` / `analyze`: is it wrong (PHPStan, Larastan; never edits). `pun check [--fix]`:
  lint fixes, then format, then analyse, in one pass; the command for CI and agents.
- **Placement:** a rule that only changes layout is format; any other edit, or a report one file can decide, is lint;
  anything that needs types or other files is analyse (plan/rules.md §1). PHPStan core and Larastan identifiers stay in
  analyse until 1.0.
- **Risky rules are lint.** `pun format` is always safe on save; on a project migrated from pint.json,
  `pun check --fix` writes the same bytes as `pint`.
- **The 40 code-deleting format fixers move to lint** (unused imports, useless else, combine issets, ...; list in
  plan/commands.md §5). They stay token fixers in the same priority-ordered pass, so Pint parity holds.
- **No `pun upgrade`.** PHP, Laravel, PHPUnit and Pest upgrades and codemods are lint groups (`lint/laravel-13/...`).
  PHPDoc type tags (`@phpstan-param`) are kept; `pun migrate --phpdoc` rewrites them (opt-in, apps only).
- **`pun lint --fix` doesn't format** (Rector doesn't); `pun check --fix` does both.
- **`pun migrate`** moves pint.json, .php-cs-fixer.php, rector.php and phpstan.neon onto pun.toml completely. It means
  config migration, not code upgrades.
- **`pun init`** asks three questions (format preset, lint sets, analyse preset), offers `pun migrate` when old configs
  exist, writes only the choices, runs one `pun check` and offers fix and baseline. `empty` is not offered (assistant's
  default).
- **No workflow wiring.** Neither migrate nor init edits composer scripts, CI, hooks or editor settings. Agent skills
  (`pun-init`, `pun-migrate`) will do that later.
- **`pun config`** prints the resolved config; **`pun explain <rule>`** a rule's docs and upstream names;
  **`pun list-files [--command format|lint|analyse] [-0]`** the files a command checks, like Mago's.
- **No LSP** until after 1.0's commands; `pun lsp`, pun-lsp and PHPantom were removed (in git history).

### Config and rules (plan/config.md, plan/rules.md)

- **pun.toml only.** Commands never read pint.json, .php-cs-fixer.php, rector.php or phpstan.neon; only `pun migrate`
  (and the harness) does. Without pun.toml: "No pun.toml found. Run `pun migrate`, or `pun init` for a new project."
  There is no `pint` argv[0] alias.
- **Rule IDs:** `<command>/<group>/<name>` in kebab-case (`format/imports/ordered`). One fixed group list shared by all
  three commands: topic groups, ecosystem groups (`laravel`, `pest`, `phpunit`, `doctrine`) and one upgrade group per
  target. Intent (correctness, style) and fix safety are data, not part of the name. Format's table: plan/format.md §9.
- **No aliases.** Renames are breaking, and that's fine until 1.0. An old or upstream name in pun.toml is an error that
  names the Pun ID.
- **Format presets:** `laravel` (default), `psr12`, `empty`. `per` and `symfony` are gone; migrate expands them.
- **Rule options are TOML-native**, not PHP-CS-Fixer's; migrate translates upstream options.
- **Lint presets are sets you tick** (`[lint] sets = ["dead-code", "modernize"]`), Rector's prepared sets; dead-code and
  modernize are init's default. Sets are tags on rules.
- **Lint levels follow oxlint:** `"off" | "warn" | "error"` per rule or group, options in a table with `level`; only
  `error` fails; enabled rules default to `error`; `--deny-warnings`, `--max-warnings <n>`, `-A/-W/-D`,
  `[[lint.overrides]]` per path. `info` is gone.
- **`--fix` applies every fix of every enabled rule**, as Rector does. No `--unsafe`, no `fix = "apply"` or
  `"safe-only"`; `fix = "off"` makes a rule report-only; safe/unsafe stays a label in the output.
- **Analyse presets:** `basic` (PHPStan 0-1) < `essential` (2-3) < `recommended` (4-5, default) < `strict` (6-8) < `max`
  (9-10), under `[analyse] preset`. There is no top-level `preset` and no shared lint/analyse tier.
- **Analyse rules are on or off**; every finding is an error.
- **No levels anywhere.** Migrate writes the exact rules an old config ran, at `error`, and when interactive offers the
  closest preset with a count of the new problems it would report.
- **Laravel, Pest and PHPUnit turn on by themselves** from composer.lock, with their version rules; a group can be
  switched off. `[php] version` comes from composer.json; the config value only overrides it.
- **`[files]`** (paths, exclude) is shared by every command, and each command's `exclude` adds to it (assistant's
  default, Mago's model).
- **Unsupported items** (an unported rule, a custom fixer class, Rector's Symfony, Doctrine and downgrade sets, a PHPStan
  extension that isn't built in, a project's own PHPStan rules): `pun migrate` names each one, skips it, asks for a PR to
  punphp/pun, and exits 0 when the rest migrated. A hand-written pun.toml naming an unknown or unported rule is a config
  error (exit 2). This replaces the older "migrate lists them and stops".
- **No PHP fallback.** Pun never runs PHP-CS-Fixer, Pint, Rector or PHPStan for a rule it lacks. Delegate mode
  (pun-delegate) is removed and no PHP is embedded (pun-php is not built). System PHP is only the test oracle.

- **One-to-one with upstream** (2026-10-17): every Pun lint rule maps to exactly one non-deprecated Rector (or risky
  PHP-CS-Fixer) rule and every analyse rule to one non-deprecated PHPStan, Larastan, strict-rules or deprecation-rules
  rule, checked against the pinned source and the tools' websites. A rule upstream deprecates, or registers wrongly, is
  not supported.
- **Rector scope for now** (2026-10-17): only the rules in the eight sets (dead-code, modernize, code-quality, types,
  early-return, privatization, naming, coding-style) plus the rector-laravel, PHPUnit and Pest sets. Rules in no set
  are left out for now.
- **All lint rules are 1.0** (2026-10-26, replaces the 2026-10-18 split): the PHPUnit and Pest lint rules
  (rector-phpunit, Pest's Rector plugin and the `phpunit`/`pest` groups) and the lint rules in no set are part of the
  1.0 bar. mapping.toml no longer has a `later` flag; all 703 lint rules are the 1.0 bar.
- **Analyse without Pest or PHPUnit support for 1.0** (2026-10-18): test files are analysed like any other PHP code,
  with no Pest plugin or PHPUnit typing (analyse 7 moves after 1.0). A project that doesn't want them analysed
  excludes them by glob: `[analyse] exclude = ["tests/**"]` removes matching files from both the analysis and the
  symbol scan (PHPStan's `excludePaths.analyseAndScan`); vendor/ is still read for symbols.
- **No upgrades below the floor** (2026-10-17): Rector's php-5.3 to php-8.3 upgrade sets and Laravel upgrade sets
  before 12 are not supported; php-8.4/8.5 and laravel-12/13 are. Pun targets PHP 8.4+ projects.

### Engines

- **Format:** the PHP-CS-Fixer token-fixer port on Pun's own lexer (not mago-syntax tokens; reference/tokens.md). The
  Mago-based formatter is gone. Files run in parallel, one worker per core. Blade stays on Node + Prettier until a
  native formatter exists (after 1.0). A Pun reprinting preset (`preset = "pun"`) comes after 1.0.
- **Lint:** rules return text edits on byte ranges found through Mago's syntax tree; format tidies the layout after.
  No php-parser printer port. Parity is `format(pun lint --fix)` = `format(rector process)`. Mago's rules become normal
  lint rules and `--mago` goes away.
- **Analyse** (assistant's call, delegated): Mago's analyser for inference, one Pun rule per PHPStan rule in the
  mapping, messages may be reworded. **Mago's issues are evidence, not rules** (Pushpak, 2026-10-18, after the spike):
  each rule is a Pun check that reads Mago's inferred types, with PHPStan's semantics for that rule and preset; a Mago
  issue is used directly only where reports/analyse-spike.md shows exactly the same check (`yes`, no extras), Laravel as a built-in Mago plugin (Pest after 1.0). Built-in symbols come from PHPStan's
  reflection (crates/pun-phpstan/data/symbols.json). Baselines are Pun's own (`pun analyse --generate-baseline`,
  pun-baseline.toml); migrate never copies a PHPStan baseline. A result cache re-checks only changed files and their
  dependents.
- **No extension system before 1.0.** Framework support is compiled in, as Mago's plugins are. Mockery, Livewire,
  Filament and the like come later as extensions.
- **Mago** from crates.io, pinned `=1.57.0`, bumped together.
- **FrankenPHP** (for the later `pun run`): link libphp through a Rust SAPI first, FrankenPHP later (assistant's call).

### Output (plan/output.md)

- **Pun's own human output** for every command, modelled on oxfmt, Biome and Ruff: file lines, problem blocks with code
  by default (`--concise` for one line), one summary line. Pint's human screen is dropped.
- **`pun check` groups by step** like Vite+'s `vp check`: one status line per step, its problems under it, one summary.
- **Brand colours** from design/tokens.json at truecolor, 256 and 16 colours; `NO_COLOR`, `--color`, ASCII fallbacks.
- **GitHub annotations** are automatic under `GITHUB_ACTIONS`; `--format=github` forces them. Clean files print nothing
  (`-v` adds a dim line).
- **Exit codes and machine formats** stay Pint- and PHPStan-compatible. Exit 2 means Pun couldn't run.
- **Agent mode:** under a coding agent every command prints compact `path:line:col rule message` lines, not JSON.

### Docs (plan/docs.md)

- **Rule docs** (2026-10-19): a page for every rule at `/docs/rules/<command>/<group>/<name>`, unported ones marked
  Planned; generated as MDX from mapping.toml, with a sync check; each rule's why and examples are fields in
  mapping.toml. A rules directory with search across upstream names and URL-kept filters at launch. One migration
  mapping page per upstream tool under `/docs/migrate/from/`, plus an overview.

### Engineering

- **Build order changes** (on top of 7a76be5): the lint and analyse engine spikes run alongside Step 1, and finishing
  level 0 waits for the analyse spike; the shared problem type is a Step 1 row; migrate 1 (format part on the new
  names) moves into Step 1; `pun init` 1-3 and 5 land in Track A, init 4 after analyse Build 1-2.

- **Audit decisions** (2026-10-18, context/AUDIT.md): a shared `[workspace.lints]` as the audit proposes, with
  clippy `--all-features -D warnings` gating CI (item 28); delete pun-phpstan's unused type model (69); fixers skip
  input PHP rejects instead of panicking (23, 24); run settings passed explicitly instead of process globals and env
  reads (26, 27), one `PhpVersion` type (53), `Token` fields private behind getters (76). Assistant's defaults, same
  spirit: typed `IgnoreEntry` (47), delete code with no production caller (71), the `md-5` crate instead of the
  hand-written md5 (88).

- **Tests:** 100% line coverage per crate; expected values from the real tools or derived by hand; no tautological
  tests (AGENTS.md).

## 3. Build order

As pushed in 7a76be5, plus Pushpak's four changes of 2026-10-17 (§2 Engineering). Steps 1 and 2 come first because every command depends on the names, the config and the output.
After Step 2, three tracks run in parallel. Each row's "done when" is in the linked Build list. Status: `todo`,
`doing`, `blocked (on what)`, `needs Pushpak`.

### Step 1. The new names and config (plan/config.md Build 1-8)

Gate passed ea7a84bc2d4c4f4b90ad177d911e6ade3accf253

**Gate:** the 7-project corpus still gives 0 differing files with `pun migrate` + `pun check --fix`, and
`scripts/check-rule-mapping.py` passes on the new names.

**Spikes, alongside Step 1.** They test the two engine decisions before rules are written on top of them.

| Work | Status |
| --- | --- |

### Step 2. The shared output (plan/output.md Build 1-6)

| Work | Status |
| --- | --- |
| output follow-up: add a warning text token to design/tokens.json that reads on a light ground (light terminals fall back to SGR 33) | todo |
| output follow-up: decide the rule docs base URL so `Renderer::with_rule_docs` (now `#[cfg(test)]`, drop that) links rule IDs, and switch `pun explain`'s docs row from the mapping.toml#L<line> link (`pun_config::MAPPING_URL`) to the rule page | todo |
| output 5: `pun check` grouped view (deferred, Pushpak 2026-10-18: parity starts without it; when built, add a row to put the analyse step in the view) | deferred |
| output follow-up: when lint and analyse emit `pun_problem::Problem` values, annotate them through `github::problem` (`::warning` for lint warnings); analyse decides whether to drop `title=` for byte parity with PHPStan's `--error-format=github` | todo |
| output follow-up (decision): under agent mode with `--format=github`, should run errors print `::error ::<message>` instead of the agent line | todo |

**Gate:** every human output sample in the plan files is pinned by a test; machine formats and exit codes still pass
`scripts/cli-parity.sh`.

### Step 3, in three parallel tracks

**Track A: format, migrate, init** (plan/format.md, plan/migrate.md, plan/init.md)

| Work | Status |
| --- | --- |
| format 4-5: memory target; 8-core benchmark in CI (laravel-framework under 1 s cold) | todo |
| format 4-5 follow-up (from bench, needs Pushpak): BENCH=format runs Pint with Xdebug loaded on this Mac; set XDEBUG_MODE=off as the analyse and lint modes do (changes format-mode behaviour of scripts/bench.py) | todo |
| analyse (from an-types2, needs Pushpak): DateTimeInstantiationRule (`new.dateTime`) needs PHP's date-string parser (timelib `date_parse()` errors/warnings). Options: port timelib's parser to Rust, or shell out to `php`. Default taken: port it (closest to upstream, no PHP runtime dependency); deferred until the rest of level 5 lands | needs Pushpak |
| format 6: swap `vendor/bin/pint` for Pun in Pinkary's CI | todo |
| audit 26 (needs Pushpak): the agent `MODE: OnceLock` in pun-cli/src/agent.rs becoming an `OutputMode` passed down from main reaches config's init.rs and migrate.rs and every reporter; the code keeps the once-per-process global (decided before any output, like a logger) unless Pushpak wants the threading. Item 88 (md-5 crate) is done | needs Pushpak |
| audit 76 follow-up (format): make `Token.id`/`Token.content` private once nothing outside pun-tokens reads them: the lint tracks' new pun-fixers files (non_printable_character, ternary_to_null_coalescing, phpdoc_to_{param,property,return}_type, analyzers/abstract_phpdoc_to_type_declaration_fixer) and pun-cli/src/migrate.rs:716-735 should use `get_content()`/`get_id()` and `Token::new`/`char`/`with_id` | todo |
| format 7 follow-up: `"header_comment": true` (no `header`) is a pun.toml config error (exit 2) in Pun; Pint configures it and reports `[header_comment] Configuration is required.` as an error on every file | todo |
| format 7 follow-up: laravel-framework pun-harness format runs exceed one 600-second heavy call; run Pint with --parallel in the harness or split the corpus run | todo |
| format 7 follow-up: with the symfony preset Pun's Blade path does not apply the symfony PHP rules Pint applies inside Blade directives (yoda_style, not_operator_with_successor_space); blocks a 0-diff symfony-preset pinkary run once phpdoc_to_comment is in lint | todo |
| init 4: one `pun check`, then offer fix and baseline | blocked (analyse 1-2) |

**Gate:** the Pinkary CI swap from `vendor/bin/pint` to Pun, and the 8-core benchmark.

**Track B: lint** (plan/lint.md)

| Work | Status |
| --- | --- |
| lint property-type coverage: close source-owned property_type.rs boundaries through public Rector contracts | doing (the refactor and parent/scalar-union correction pass 52 unchanged tests before the behavior change, then all 78 combined-main tests with 25/25 changed executable lines covered; same-class named-static and typed-peer/inherited constructor detection pass the complete combined-main gate with all 153 changed lines covered; the integrated ClassView refactor covers all 550 corrected owner lines; the subsequent constructor/property producer refactors pass all 273 combined-main tests with 82/82 changed executable lines covered, preserving all 10,084 test/fixture files; both original property consumers have complete coverage; current property_type.rs coverage is 1,165/1,212, leaving 47 lines; parse_url precision/order and constant-array PHPDoc probes remain red) |
| lint delivered source coverage: close the complete measured suite's remaining source-owned gaps | doing (the original complete measurement passes 631 tests at exact bd8a29649; the current source-matched cohort union includes fresh coverage for all nine constructor/property sources from 273 passing combined-main tests; raw coverage is 72,930/73,459, leaving 529 lines, or 481 after existing exclusions; seven shared files cover 2,779/2,788 lines, leaving facts.rs:940, types/mod.rs:531 and type_factory.rs:411-417; the last seven are reached only by a retained failing late-static probe and do not count as passing coverage; no new exclusions) |
| lint 1-2 follow-up (from perf lint 2): when `pun lint` is wired into pun-cli, check files while pun-workspace's finder is still walking (shared LIFO work stack, as in examples/check.rs) instead of discovering every file first, and take the entry type from `DirEntry::file_type` instead of a stat per entry | todo |
| lint 1-2 follow-up (from perf lint 4): when `pun lint` is wired into pun-cli, read files on a few threads of their own (3 in examples/check.rs) and parse on all cores, instead of opening on every core; opens on more than 4 threads contend on macOS | todo |
| pun-lint: replace `syntax::same_code` with a structural node comparator (Rector's NodeComparator), shared by rules | todo |
| pun-lint: name resolution for rules that need resolved names (mago-names or analyse's index) | todo |
| lint: the engine uses `pun_problem::{Problem, apply_fixes}` for diagnostics and the fix loop | todo |
| lint 3-6 follow-up (platform): a token-path (PHP-CS-Fixer) lint rule reports its file at the first changed line, not one diagnostic per change; split its diff into per-rule hunks so each change is a problem with its own span and `@pun-ignore` line forms work for it (today only `@pun-ignore-file`) | todo |
| lint 3-6 follow-up (platform): Pint's checkstyle, junit and gitlab reporters list a file's lint and analyse problems by rule ID only (PHP-CS-Fixer's per-file shape); give each problem its line and message there | todo |
| platform decision (lint 1-2, from config 7): `pun lint`, the lint half of `pun check` and `pun analyse` skip Blade templates and Pint's default directories (bootstrap/cache, build, node_modules, storage) besides their own excludes, as Mago and PHPStan see only PHP sources; `pun lint --mago` is gone (its Laravel rules are native lint rules) | done |
| AUDIT item 28: add `[lints] workspace = true` to each crate's Cargo.toml and fix the clippy hits (`cargo clippy -p <crate> --all-targets --no-deps`); pun-cli, pun-workspace and pun-problem already opt in | todo |
| pun-analyse: a `run` that takes the analysed files' sources, so `pun check` reads each file once instead of again after format and lint | todo |
| platform bug (found by salvage-cli): a token lint rule at `warn` (e.g. `style/no-alias-function`) makes `pun lint` agent output say `fail, 1 file with fixes` and the human summary mark the file with ✖, while the exit code is 0 (warnings pass); report such files as warnings (`pass, 1 warning`, `[warn]` on the line) as native warnings are | todo |
| lint vs Rector follow-up (scripts/lint-vs-rector.py, laravel-framework: 5427 (file, rule) pairs agree, 336 Rector only, 146 Pun only; pinkary: 0 differences): the `types` rules' residuals. Rector only: add-return-type-from-new 67, add-closure-never-return 54, add-array-return-type 41, add-return-type-from-constant 26, add-string-return-type 17, add-arrow-function-return-type 15, add-numeric-return-type 13, add-return-type-from-param 11, add-fluent-return-type 9, 4 each add-string-return-type-from-scalar/add-return-type-from-property/add-bool-return-type. Pun only: add-string-return-type 31, add-return-type-from-cast 26 (e.g. `PhpRedisLock::release()`, which overrides the untyped `RedisLock::release()`: port `ClassMethodReturnTypeOverrideGuard`'s parent and child checks now that the codebase has the project's classes), add-closure-void-return 14, add-closure-return-type 7 | todo |
| lint framework batch-2 comparison: finish bounded positional-file runs against the captured Rector output without changing the full configured ProjectCodebase | doing (3,024/3,027 files compared on the lint-batch-2 binary: 6,304 agreeing pairs, 75 Rector-only and 93 Pun-only; normalized main now completes the three previously stalled files in 174–206 seconds, with 22 agreeing pairs and one Pun-only cast-return finding; all 3,027 inputs remain byte-identical; this mixed-snapshot evidence does not establish full current-main parity) |
| shared object-intersection normalization: remove redundant proven supertypes and duplicate nested conjunctions without dropping mixed exclusions | doing (all four regressions from the initial 351-job gate now have integrated source-owned repairs; 57 combined-main object, native and string controls pass without changing existing expectations; exactness is retained by shared Analyzer artifacts but not Pun's converted PHPStan object types; the nine-line analyser probe improves from a 300-second timeout at 5,957,032 KiB to 3.48 seconds at 137,852 KiB, matching PHPStan's empty diagnostics; fresh combined gates and whole-crate coverage are running) |
| shared explicit-mixed native inference: feed existing TMixed explicitness from native/PHPDoc declarations, builtin results and offset reads into lint's native facts | doing (receiver-aware native property/copy/offset/capture inference passes the complete combined fixture/public/CLI gate and source-matched coverage; dual-version PHPStan captures are byte-identical; conditional overwrite and array_pop(string) remain boundaries; no lint consumer change yet) |
| lint vs Rector follow-up (needs Pushpak): `style/require-statement-padding` reports 60 laravel-framework files Rector leaves: Rector runs every rule in one traversal, so `NewlineAfterStatementRector` never sees the statements `RemoveAlwaysElseRector` moves out of an `else`; Pun's later fix round does and adds the blank line (a second `rector process` would too). Kept: rounds as in Ruff (lint.md); the other option is to stop a rule from reporting on code an earlier round's fix wrote | needs Pushpak |
| lint vs Rector follow-up: `complexity/prefer-empty-array-identical` misses 34 laravel-framework files where PHPStan knows a written parameter is still an array (`$credentials = array_filter($credentials, ...)`, then `empty($credentials)`): syntax_types.rs `parameter` gives up on any write before the code; take the written value's type from facts.rs instead | todo |
| lint vs Rector follow-up: one laravel-framework file each, Rector only: php-8.4/prefer-round-mode-enum, php-8.4/prefer-array-any, dead-code/no-useless-ternary, dead-code/no-unused-expression, dead-code/no-unreachable-code, dead-code/no-return-in-constructor, complexity/prefer-min-max, complexity/no-sole-value-sprintf, complexity/no-boolean-literal-compare, classes/no-protected-method-in-final, types/add-numeric-return-type-from-scalar, types/add-never-return-type; Pun only: types/add-function-void-return (`scripts/lint-vs-rector.py laravel-framework` lists the files) | todo |
| coverage: context/reports/COVERAGE.md has 75 exclusions (about 55 from the pun-lint coverage pass: non-target node guards, no-Program-ancestor and impossible-scope lines); AGENTS.md wants them rare: turn each guard into a `let ... else { unreachable }`-free shape (match on the target node in the rule's visitor, or delete the guard) so the line disappears instead of being excluded | todo |
| lint 7: all 703 lint rules native, in usage order. 703 of 703 mapping.toml lint rules ported (count: `python3 -c` over `[rules.lint]` `ported`). Ported rules retain the corpus residuals above | done |
| lint Laravel attributes batch 8 (needs Pushpak): authorize integration of the independently produced 32-rule mailbox; keep strict upstream parity as the default | needs Pushpak (read-only audit complete; both mailbox digests match; all 154 upstream fixtures and 67 Illuminate stubs are byte-identical, with 23 Oracle and six Pun controls; source changes must replace the tab-indentation and `new static` skips with upstream output; malformed-array parser outcomes need a separate check; batch 8 remains unapplied and excluded from counts) |
| lint Pest cache dependency: invalidate no-redundant-uses results when the applicable tests/Pest.php changes | todo |
| lint 7 types: `add-return-type-from-ternary`, `add-nullable-return-type`, `add-union-return-type` (src/rules/types, out of `all()`) pass Rector's fixtures but differ on laravel-framework; port Rector's `TypeFactory::uniquateTypes` and PHPStan's arithmetic result types, then add them to `types::all()`; then typed-call, native-call, new-array, private-method-array, try-catch, from-parent, then the param, property and phpdoc rules. Corpus residuals come from Rector's cross-file effects, vendor classes, declared-only `mixed`, docblock shapes and Mago's truthy narrowing | todo |
| lint 7 next small rules: classes/add-dynamic-properties (property types and docblocks), phpdoc/no-useless-{param,return,var} (TypeComparator), functions/no-extra-arguments and require-strict-in-array (types), arrays/no-implicit-array-creation (needs PHPStan's maybe-defined scope through loops), functions/require-array-filter-callback and the six style/no-dynamic-* strict rules (messages print PHPStan type descriptions), dead-code/no-unused-private-method and no-empty-method (CallCollectionAnalyzer and ArrayCallableMethodMatcher are type-based), laravel/prefer-latest-oldest (QueryBuilderAnalyzer needs Laravel types). Type-led ones (ExplicitReturnNull, SimplifyIfReturnBool, InlineArrayReturnAssign, SimplifyIfNullableReturn, VariableConstFetchToClassConstFetch, ThrowWithPreviousException, SafeDeclareStrictTypes) wait for Build 8 | todo |
| lint 7 follow-up (from next small rules): no-for-init-overwrite and no-loop-variable-overwrite read definedness from the syntax: a by-reference argument (`preg_match($p, $s, $m)`) and a `try` whose body cannot throw do not define a variable in Pun, as PHPStan's signatures and throw points make them | todo |
| lint 7 follow-up (from next small rules): no-unused-private-constant, prefer-spread's constructor-assigned property types, no-useless-variable and prefer-catch-name-from-type: Pun's fix rounds go on where Rector stops after one pass per file (a chain of constants only read by each other all go; `$x = $x; return $x;` after `$x = new self()` ends as `return new self();`) | todo |
| lint 7 follow-up (from next small rules): rules reading the facts codebase see no doc comments (it is built `without_docblocks`): no-parent-call-without-parent removes a call a parent's `@method` alone backs, require-strict-function-call reports a value only a `@param true` types; prefer-spread does not follow `$this->{'name'}` to its property | todo |
| lint 7 follow-up (from next small rules): laravel/prefer-casts-method is `fix = "safe"` in mapping.toml but Mago 1.57.0 has no fix (rector-laravel's ModelCastsPropertyToCastsMethodRector is in the unsupported laravel-110 set); decide whether Pun writes the `casts()` method | todo |
| lint 8: expose expression types, class subtyping and resolved constant values to pun-lint rules (Context has only source/path/php_version); blocks the type-aware rector-laravel ports and the rows above that wait for analyse's index | todo |
| lint: `PreferNullsafe` (optional/nullsafe, written, withheld): implement `Rule::configure` for its `exclude_methods` option, then add it to `rules::all()` and mapping.toml | done |
| lint: pun-fixers ports `lint/types/add-void-return-type` (`void_return`) and `lint/types/require-strict-param` (`strict_param`), token fixers like `strict_comparison` | todo |
| lint: switch crates/pun-cli/tests/cli.rs:635-659 (`no-homoglyph-names`) and tests/migrate.rs:196 (`no_null_property_initialization`, `phpdoc_to_comment`) to an unported rule as the unported example; those rules are ported now | todo |
| lint 7 follow-up (from coverage): `add-return-type-from-ternary` at PHP 7.1 types `return $x ? get_class($x) : get_class($x);` as `: string`; Rector (ReturnTypeFromStrictTernaryRector alone) leaves it (PHPStan's `get_class()` is `string|false` there). Add the Oracle fixture and match | todo |
| lint 7 follow-up (from coverage): whitespace after a removal differs from Rector: removing the sole statement of a `switch` case leaves a blank line, and `no-empty-loop` on a braceless loop body leaves different whitespace. Compare with .oracle/rector-bin and match | todo |
| lint 7 follow-up (from coverage): the Pun/ fixtures that keep Pun more conservative than Rector (ArrayAccess objects and written typed params in dead-stmt/empty-array rules, file-level variables, a leading run in ForeachToInArray, a self-calling private method's param, `get_class()` closures typed `string|false`, `never` on an unknown interface). Revisit each once lint 8 gives rules types | todo |
| lint 7 follow-up: `php-8.5/no-null-array-key` (ArrayKeyExistsNullToEmptyStringRector) needs the key's type; port it once analyse's types reach pun-lint (Build 8) | todo |
| lint 7 follow-up (from batch 2): move crates/pun-lint/src/rules/rector_code_quality.rs's shared helpers (precedence table, in_parent, statements_edit, inverse_node/de_morgan, call_to/args/is_constant/equal) into `crate::syntax` for later batches (syntax.rs is owned by lint 1-2) | todo |
| lint 7 follow-up (from batch 2): `lint/complexity/prefer-null-coalesce`'s mapping summary describes more than ArrayKeyExistsTernaryThenValueToCoalescing (isset ternaries, if/return chains, foreach lookups); check whether more upstream rules belong to it | todo |
| lint 7 follow-up (from batch 2): on a colon-body `if (...): ... endif;` Rector reprints the `if` with braces when one statement becomes two, Pun keeps the colon syntax | todo |
| lint 7 follow-up (from lint parity): `pun lint` with every rule on panics (index out of bounds, rector_stmts.rs `with_comments`) in `require_strict_types_in_tests::declare_strict_types` on a file holding only `<?php`; with all 703 ported, the `rule not supported by Pun yet` path in pun-cli (lint_plan.rs, run.rs `unported_error`) is unreachable and its CLI tests are removed, so delete that code or keep it behind a test hook | todo |
| lint 7 follow-up (from batch 1): port `lint/dead-code/no-empty-loop` (RemoveDeadLoopRector) once analyse gives pun-lint pure-function and Throwable checks (Build 8) | todo |
| lint 7 follow-up (from batch 1): decide in the engine whether Rector-sourced rules apply one pass per file, as Rector does, instead of fix rounds until no fix is left (`if ($a) {} else {}` is `if (!$a) {}` in Rector, removed in Pun) | todo |
| lint 7 follow-up (from batch 1): rewrite an emptied alternative-syntax body with braces in rules/php_parser.rs's shared remove(), so no-empty-block and no-useless-catch match Rector as no-useless-continue does | todo |
| lint 7 follow-up (from batch 1): examples/fix_dir.rs runs `rules::all()`, which now holds the batch 1 and 2 rules; check that this is intended | todo |
| lint 7 follow-up (from batch 1): merge rules/php_parser.rs and rules/used_variable.rs with batch 2's rules/rector_code_quality.rs helpers when they move into `crate::syntax` | todo |
| lint 7 follow-up (from batch 3): port `lint/control-flow/prefer-returned-condition` (SimplifyIfReturnBoolRector) once analyse inference gives the condition's type (ExprBoolCaster) | todo |
| lint 7 follow-up (from batch 3): port `lint/complexity/prefer-inline-array-return` (InlineArrayReturnAssignRector) once class-constant and magic-constant values resolve (ValueResolver) | todo |
| lint 7 follow-up (from batch 3): merge rules/rector_stmts.rs (a copy of php_parser.rs's token helpers) with batch 1's rules/php_parser.rs and rules/used_variable.rs | todo |
| lint 7 follow-up (from batch 3): laravel-framework tests/View/Fixtures/section-exception.php: Rector reprints the whole `<?php ... ?>` file, Pun removes only the unreachable statement and its `?>` (a Pun/ fixture); no-else-after-return and no-identical-branches match Rector only after format on colon `if`s and single-statement bodies | todo |
| lint 7 follow-up (from batch 4): when a `switch` with only a `default` is a whole function body, or an `if`+`return null` pair is a whole `if`/loop body, php-parser reprints the parent; Pun does so only for no-redundant-null-check's function bodies (context/plan/lint.md §7 batch 4) | todo |
| lint 7 follow-up (from rules2 batch): skipped as type-led: laravel AppEnvironmentComparisonToParameter, AssertStatusToAssertMethod, CallOnAppArrayAccess; also ValidationRuleArray (printing), RemoveTestsOverriddenPrivateMethodParameter (rewrites callers), RemoveParentDelegatingConstructor (parent type mapping), and RemoveDeadIfBlock vs constant-condition (which mapping.toml row it belongs to) | todo |
| lint 7 follow-up (from rules2 batch): known gaps against Rector: prefer-request-facade leaves float keys alone; add-param-type-sprintf treats only a `match` with 2+ earlier arms as narrowing; no-unused-promoted-property counts a fetch of the name on any object as a use (Rector only the class's own type); prefer-travel-to has no Oracle (Rector cannot reflect Laravel's TestCase without Laravel installed); prefer-class-keyword leaves a string whose other parts are not UTF-8 alone (Rector writes the bytes raw) | todo |

**Gate:** `format(pun lint --fix)` equals `format(rector process)` on Rector's fixtures and the corpus, for every rule.

**Track C: analyse** (plan/analyse.md)

| Work | Status |
| --- | --- |
| analyse 3: finish level 0 and the level-2 class rules; the current rule counts are in §0. Not yet: DebugScopeRule, DumpNativeTypeRule and FileAssertRule | todo |
| analyse level 0 named arguments: complete MethodCallWithPossiblyRenamedNamedArgumentRule before activating its mapping | doing (the corrected snapshot passes all 2,787 level cases, 1,315 rule cases and 782 public/library tests; the expanded mixed-policy generic-proxy regression matches all thirty records and the CLI matches all four JSON records; source-matched coverage is complete; ordinary compound semantic argument/reference/template checks remain explicitly guarded, so the mapping stays inactive) |
| analyse compound method signatures: retain combined semantic parameter, reference, template and return metadata through the ordinary shared argument checker | doing (queued source-identity bridge matches 129 public declaration/annotation records across 25 configurations, all 132 TemplateMap combinations, 32 private identity records and 128 transformation records; its identity commit covers 1,361/1,361 changed production lines without exclusions and passes 106 PHPStan tests, 447 native units and 146 analyser tests across 28 targets; fresh oracle recaptures preserve every containing Methods/callable family: 571 cases and 2,787 diagnostic records; six stale level gaps are confirmed; the queued singleton callable conversion repair matches all eighty original callable records and twenty mutation controls with 3/3 changed lines covered, while broader conjunction probes remain red; the multiple-array array_map producer remains separate; main integration also needs downstream lint and no-op unset adapters; traversal/export prerequisites pass the complete main gate; staticness is separately verified and queued, while actual body-local purity, compound activation, variance and void authority remain unfinished; no private red ancestry) |
| analyse native list source kind: retain constant-shape identity separately from generic lists carrying written or asserted offsets | doing (the queued native-kind prerequisite passes seven owning tests, 446 native analyser units, 304 codex units and 61 neighbors; all 100 changed executable production lines are covered privately without exclusions, with seventeen external-function export warnings retained; main integration must adapt the two newer unset_member sites before combined verification; the shared converter remains separate; no kind inferred from equal values, count or call syntax) |
| analyse open-shape template bound validation: substitute both known entries and the remaining value tail with the matching comparator contract | doing (the isolated definition_type_replacer trial repairs two missing native class-bound checks but adds an incorrect closed-shape rejection; PHPStan gates extra-key comparison on both explicit unsealed carriers; a coherent traversal/comparator correction is in progress; actual invocation inference already traverses prefix and tail and its twelve-record value family is green before and after; shared generic-checking diagnostics remain separately incomplete) |
| analyse array_map return inference: wire genuine callback invocation into the PHPStan-rule provider registry | doing (native selected-acceptor callbacks and omitted-unsealed-key producers pass the combined main analyser, lint, selected original-fixture, CLI and coverage gates; all 143 changed executable production lines are covered without exclusions; the independent fifteen-record PHPStan 2.3.1 matrix remains at two exact matches and thirteen differences because registry activation is unchanged; all 126 original Methods/ReturnTypeRuleTest cases pass with existing allowances, including testBug12223, which was already green on main; activation still needs source-owned open-list conversion and the reflected builtin fallback for required named-argument holes, including the extension's non-array operand branch; no callback syntax heuristics or blanket provider fallback) |
| analyse generic-list literal writes: retain list certainty only where AccessoryArrayListType::setOffsetValueType does | todo (the complete retained open-list control shows that writing offset 2 after offset 0 needs a general nonnegative-key array with both offset constraints, not a native TList; separate from constant-shape classification) |
| analyse open constant-array slicing: use ConstantArrayType::sliceArray rather than the generic-array tail path | todo (the complete retained open-list control expects array{int} for array_slice(list{int, ...<string>}, 0); current main returns non-empty-list<int|string>; source kind and slice semantics are separate prerequisites) |
| analyse isset diagnostics: complete IssetCheck and whole-rule activation | doing (live markers, the corrected checker bridge and bounded helper summaries pass the complete combined main fixture, CLI and source-matched coverage gate; private facility coverage is 250/250 changed lines; the corrected bridge matches 129 complete records, passes all six library tests and covers 41/41 changed executable lines; direct-helper captures match 159 diagnostic records; the compound-selection follow-up passes 79 tests, matches 31 marker and 114 diagnostic records, and brings cumulative helper production coverage to 220/220; tied distinct declarations and ambiguous carriers remain conservative; a pruning-only failure was not reproduced; native certainty configuration remains separate; mapping stays inactive) |
| analyse by-reference capture feedback: complete local and escaped invocation semantics | doing (read-only captured values, reference feedback and unused-capture flow pass the complete combined gate and coverage; all 23 freshly recaptured full/native records now pass without the obsolete scalar/array unavailability allowances; broader escaped invocation semantics remain) |
| analyse native certainty observations: preserve native argument values through checking calls, native object returns and diagnostic-only dumps before completing PHPDoc-certainty tips | doing (generic-property descriptions, local coalesce purity, callable-array metadata and the strict consumer are integrated locally; all 108 private complete comparisons pass through both constructors, covering 888 records without filtering, seven raw observations and 29 once-only events; consumer changed coverage is 20/20 production lines plus one required test literal; no replay or full/PHPDoc fallback; combined-main verification is running; broader certainty/tip options remain) |
| analyse nonliteral initializer order: retain insertion order through array-union and conditional class-constant initializers | doing (the direct correction passes the complete combined main gate and coverage; the queued scoped provenance correction matches 76 complete records and covers 206/206 changed lines; private conditional addition matches 127 records and covers 160/160 changed production lines; its exact-key correction matches 92 same-value records and forty unchanged controls, passing 91 tests with 5/5 changed lines covered; the queued public 256-alternative fallback matches 259 complete records, passes 93 tests and covers 62/62 changed production lines, with its generic return reached 27 times by passing public inputs; its owner now handles native-only insertion order; global order, numeric-string arithmetic, dynamic PHP_OS keys and broader oversized/open arrays remain separate) |
| analyse boolean-membership narrowing: preserve PHPStan's alternative constraints rather than deciding runtime membership | doing (the queued correction matches all nineteen complete level-4 records through both constructors; 53 tests and all 89 owning cases pass under the unchanged 31 gap entries; exact changed production coverage is 79/79; failed reference-array reads now pass the main gate, while their missing offset diagnostic still needs the separate level-3 read rule) |
| analyse parameter native/PHPDoc nullability: clamp documented nullable objects against nonnullable native parameters before nullsafe observations | doing (the queued bounded object-entry port matches all 91 diagnostic records and 22 observations, with 21 native answers supported and one class-union control unavailable; changed production coverage is 31/31; integration must update only the three newly supported nullsafe controls and recapture their positions) |
| analyse native-object PHPDoc shapes: retain object-shape refinements on parameters declared `object` | todo (the untyped shape controls retain the shape, but native-object inputs erase it; independently captured dumps remain in /tmp/pun-sentinel-verification-evidence) |
| analyse pure-void diagnostics: report void.pure for pure functions whose bodies only unset values and dump types | doing (generic-union unset transfer and non-ArrayAccess unset effects pass the frozen main gate: 144 analyser tests, 100 lint tests, all 405 original-fixture partitions and 57 CLI tests; the remote union-method integration also passes 169 lint tests and all sixteen lint CLI tests; fresh complete captures preserve sixty producer records and seven effect records, with 161/161 changed production lines covered; the isolated no-op source-child correction matches 21 identity records and sixty complete diagnostics, passes 85 public tests and 447 native units, and covers 18/18 changed lines; its three private offset-read purity reds come from lacking main's ArrayAccess-No read prerequisite, not an established main regression; identity integration, reference/property and whole-rule boundaries remain) |
| analyse exit match conditions: report match.alwaysFalse when an exit expression terminates a match condition | todo (the yielded-condition delivery control passes, while the exit-condition probe misses the upstream record; failing comparisons remain in /tmp/pun-sentinel-verification-evidence/refined.log) |
| analyse destructuring diagnostics: port ArrayDestructuringRule's four failed-offset diagnostics independently of ErrorType assignment recovery | todo |
| analyse invalid-generator recovery: preserve mixed property values when erroneous PHPDoc prevents a yield send type | todo (PHPStan's public probe returns mixed; source predicts a never fallback, but combined Pun reproduction remains) |
| analyse independent argument options (needs Pushpak): choose Pun option ownership for checkFunctionArgumentTypes, checkArgumentsPassedByReference and checkThisOnly without coupling their values to one aggregate rule level | needs Pushpak (default: preserve independent analyser API overrides and level-derived preset defaults; reject unrepresentable migration overrides explicitly rather than changing the level or silently losing settings) |
| analyse generic class-string bounds: preserve the source template's bound in class-string conversion | doing (main's original bottom-bound probe remains red and converter line 643 remains uncovered by passing runs; PHPStan retains raw Never tags for unsupported-bound diagnostics but its effective factory falls back to explicit Mixed with template identity; all sixteen unsupported-bound records pass privately in the verified source-identity delivery without accepting ordinary class-string<never>; broader bridge regressions and integration remain, so no main gap allowance) |
| analyse bounded dynamic class strings: preserve literal-string provenance for object ::class without changing PHPDoc or get_class results | todo (the complete retained public input has one remaining diagnostic mismatch after the verified missing-parent correction; source inspection locates lost bounded provenance in Mago's magic-constant producer and OfType/Generic representation, not Pun's description; templates, generic arguments and finality require separate contracts; no formatter suffix or blanket converter change) |
| analyse prerequisite source coverage: restore complete source-matched execution after the native-certainty and dynamic-callable integrations | doing (the complete current fixture gate, 242 analyser tests, 92 PHPStan tests, 135 lint consumers and 57 CLI tests pass; all 63 new prerequisite production lines are covered; source-matched analyse coverage is 44,359/44,360 after existing exclusions, leaving only genuinely red bottom-template converter line 643; PHPStan is 10,780/10,780 and lint 72,881/73,382; all 77 fresh exact-object exports are warning-free; no failed profiles or new exclusions) |
| analyse level 0 empty-offset reads: finish OffsetAccessWithoutDimForReadingRule parity through the existing assignment-context checker | doing (the mapping is already marked ported; expression-keyed assignment scopes pass the complete frozen main gate and match the fresh complete PHPStan capture; private verification matches 77 records in each mode and the whole-rule probe's 32/32 records, with 108/108 changed-line coverage; the separate conditional-callable probe remains 3/2 and loses two level-5 purity records because shared Signature drops purity/staticness and collapses two raw acceptors; raw reference merging is disproved by both-impure controls; 64 neighbors pass; full parity remains blocked, without mapping changes or diagnostic filtering) |
| analyse level 0 arrays: complete DuplicateKeysInLiteralArraysRule beyond the verified scalar/constant keys, sealed unpacking and byte-exact literal keys; identifier boundaries still use UTF-8 text | doing (shared quoted-key decoding, byte-based tokenization and comment ingestion pass the complete combined gate and coverage; identifier handling and whole-rule activation remain incomplete) |
| analyse level 0 variables: port DefinedVariableRule and CompactVariablesRule on shared variable-certainty observations, including assign/undefined-allowed contexts, dynamic names and the CLI/maybe-undefined switches | doing (top-level unset and counterfactual !isset invalidation match 67 and 119 complete diagnostics on main; the combined native-scope gate passes 119 public/library tests, 52 rule cases and 94 level cases with 109/109 changed lines covered; genuine absence and local behavior remain; general scope joins and compact's independent selector decision remain separate; no whole-rule activation) |
| analyse level 0 unset: port UnsetRule's readonly and hooked-property checks, recursive offset checks and definitely-undefined local checks through shared reflection, types and certainty | doing (27 integrated public tests and ten exported cases pass; required-interface native reflection and annotation writable types are integrated; fresh captures cover the bounded reflection contracts; complete rule activation and combined coverage remain) |
| analyse level 0 attributes: complete the twelve existing attribute-rule contracts against AttributesCheck, including constructor types and version/named-argument controls, before activating their mappings | doing (explicit aliases, parenthesized flags and shared trait-constructor reflection integrated; isolated reflection controls match 9/41/45 oracle diagnostics at levels 0/5/6; conditional/coalesce initializer inference remains isolated; combined gates running; mapping stays unported) |
| analyse shared class names: carry imported-name and internal-class case options through the shared ClassCaseSensitivityCheck, preserving explicit aliases and written-name distinctions | doing (isolated source port against fresh PHPStan controls; qualified special-name parser validation and source-only class_alias provenance remain separate attribute boundaries) |
| analyse compact rule selection (needs Pushpak): variable.undefined has one canonical Pun rule, so compact findings cannot be independently enabled, disabled or ignored by analyse/undefined/compact-variable | needs Pushpak (default: preserve the existing canonical identifier decision; keep compact unported while its extraction and public selector boundaries remain) |
| analyse initializer dependency cycles: extend canonical resolution beyond the currently supported global, ordinary-class and trait dependencies | todo |
| analyse dynamic initializer selectors: preserve PHPStan's mixed fallback for expression constant names such as `A::{'V'}`, separately from ordinary expression flow | todo |
| analyse class constants: extend ClassConstantRule's receiver checks beyond named classes and self/static/parent to inferred expression receivers at higher levels | doing (named-call identities and bounded purity invalidation integrated; nine public tests pass; dynamic/nullsafe calls, indirect variables, dynamic property selectors and general type-dependent comparisons remain; mapping stays unported) |
| analyse level 0 includes: extend integrated RequireFileExistsRule and IncludedFilePathResolver beyond local files to supported stream paths before whole-rule activation | doing (local phar:// archive members and gzip restart boundary integrated; seven include tests and fresh PHPStan capture pass; runtime wrappers, aliases/mounts, OpenSSL signatures and bzip2 remain unsupported) |
| analyse native types (needs Pushpak): exact `getNativeType()` needs full/native state on the same control-flow walk; Mago 1.57 exposes one lane and no PHPDoc-disable setting. Choose a maintained Mago source patch or a Pun-native scope engine. Default: preserve PHPStan's paired-state contract and leave native-dependent debug rules unported rather than substitute a comments-disabled second pass | needs Pushpak |
| analyse callable inference: backport Mago 1.59.0's `Closure::fromCallable()` provider through Pun's production registry, preserving PHPStan signatures through assignment and invocation | doing (integrated locally; combined verification running; unknown/static-closure spelling and unresolved-string union fallback remain documented boundaries) |
| analyse callback follow-up: foreach-defined coalesce return values lose their raw signature (`list<mixed>` rather than oracle `list<1|23>`); callback-refined constant conditions also lack the native/PHPDoc tip | todo |
| analyse 3 follow-up: extend inline `@param-closure-this` bindings to dynamic callables, closure variables, constructor and unpacked callbacks, and template/conditional/union bindings | todo |
| analyse 3 follow-up: method-call declaring-class displays outside the declaring class's own scope omit template arguments; use default bounds and resolved ancestor template maps, not just the in-scope templates | todo |
| analyse 3 follow-up: receiver refinement beyond literal method_exists guards: inferred singleton-string method names, property_exists accessories and compound receiver types | todo |
| analyse 3 follow-up: the internal/deprecated class-name extensions (RestrictedInternalClassNameUsageExtension, RestrictedDeprecatedClassNameUsageExtension) at their PHPDoc-tag locations (@var/@param/@return/@throws/@mixin/@method/@property/@extends/@implements/@use/@template bounds/type aliases), property-hook parameter and return types, and `@method static` calls (calls.rs: a static call to a missing method gets no member) | todo |
| analyse 3 follow-up (deprecation-rules): the deprecated-class-scope check covers the enclosing class, trait and function/method only (not the resolvers a project registers as deprecatedScopeResolvers; `#[\Deprecated]` is unchecked); CallWithDeprecatedIniOptionRule reads a string-literal first argument only (`$scope->getType()` constant strings upstream); a branch a `PHP_VERSION_ID` guard rules out is skipped by FetchingDeprecatedConstRule only (PHPStan analyses no dead branch); once an assignment narrows `$var->prop`, Mago no longer types `$var` there, so the member rules (internal.rs `narrowed_object()`) reuse its latest typed occurrence in the function; a built-in method's or function's `@deprecated` description (phpstorm-stubs, e.g. Elastic\Apm\ExecutionSegmentInterface::getDistributedTracingData()) is not in pun-phpstan's symbols.json, so its message lacks it | todo |
| analyse 3 follow-up (strict-rules): ClosureUsesThisRule, RequireParentConstructCallRule, WrongCaseOfInheritedMethodRule and DynamicCallOnStaticMethods(Callable)Rule run under their `strictRules.*` switch, read from the PHPStan config only (pun.toml has no switch yet). Not applied: the parameters strict-rules' rules.neon changes (checkFunctionNameCase, so the call-site `method.nameCase`/`staticMethod.nameCase`/`function.nameCase` checks; checkDynamicProperties, polluteScopeWith*, checkInternalClassCaseSensitivity, reportMaybesIn*); DynamicCallOnStaticMethodsRule's PHPStanTestCase/TypeInferenceTestCase prototype exemption | todo |
| analyse 3 follow-up (do-while condition types): infer.rs `after_expression` unions every loop evaluation of a wanted expression, and Mago's re-passes also apply a do-while condition at the top of the pass against the merged loop context. So `do { $o = null; } while ($o);` types the condition `stdClass\|null` where PHPStan has `null`, which breaks `doWhile.condNotBoolean`'s message and hides `doWhile.alwaysFalse`. Keep only the after-body application for the condition span without losing the do-while exit the union protects (property initialization) | todo |
| analyse 3 follow-up (3n): ConstantRule's `__COMPILER_HALT_OFFSET__` counts any `__halt_compiler` in the file (PHPStan: a top-level one); `defined()`/`define()` guards are kept per branch, not merged through scopes; UninitializedPropertyRule: no `additionalConstructors` or ReadWritePropertiesExtension, and only `$this` methods declared `never` end a flow; DuplicateKeysInLiteralArraysRule's union and variable keys (gaps.txt) need inferred key types | todo |
| analyse 3 follow-up (3o): FunctionCallParametersCheck's named arguments (argument.missing/unknown/duplicate/positionalAfterNamed/unpackAfterNamed/nonUnpackAfterUnpacked/namedNotSupported) run; left: a user parameter's type in `Missing parameter` is Mago's type id read as PHPDoc, combined variants union their type descriptions as text, built-ins' named-argument variant selection (gaps InstantiationRuleTest::testBug7048, CallToFunctionParametersRuleTest::testImplodeNamedParameters) and the scope's narrowed PHP version (testDisallowNamedArgumentsInPhpVersionScope) | todo |
| analyse 3 follow-up (property hooks, level 3): SetNonVirtualPropertyHookAssignRule follows a set hook's flow by statement: branches and loops merge (a loop runs zero or more times, constant conditions are not evaluated), only a `throw`/`exit` statement ends a path (not a call to a `never` function), `try`/`switch` count an assignment anywhere inside as certain, a method call on `$this` does not invalidate the assignment (PHPStan forgets it), and a statement that ends the hook counts with the scope before it | todo |
| analyse 3 follow-up: ReadingWriteOnlyPropertiesRule and RestrictedClassConstantUsageRule check `$this` only (RuleLevelHelper's checkThisOnly, levels 0-1); from level 2 they need any expression's type (findTypeToCheck) | todo |
| analyse 3 follow-up: MissingReadOnly(ByPhpDoc)PropertyAssignRule records the virtual-assign writes (by-reference arguments, FuncCallHandler's sort/array_pop/... array argument) only for function calls in the class's own file: not method or static calls with a by-reference parameter, not calls inside a used trait's file, not `$this->p[...]` arguments | todo |
| analyse 3 follow-up: level-0 corpus residuals, pinkary 0 / larastan 6 / pest 5 / laravel-framework 0 (378 matching): ignore.unmatchedLine where the ignored error's rule is not ported yet (larastan 6, pest 5) | todo |
| analyse dead-code: native DefineParametersRule, UselessFunctionReturnValueRule, method/static NoDiscard, the trait condition, argument.unused, output and discarded method checks; batch 1 adds unused closure use / constructor parameter (featureToggles.reportPreciseLineForUnusedFunctionParameter through Project, Checks and Checker), finally.exitPoint (synthetic jumps report line -1; Error.line is i64), UnreachableStatementRule, UnusedPrivate{Constant,Method,Property}Rule, plus UnusedLabelRule and NotAnalysedTraitRule from analyse-l0. Batch 2 adds NoopRule and CallTo{Function,Method,StaticMethod,Constructor}StatementWithoutSideEffectsRule (a syntactic model of impure points). Batch 3 adds the constant condition rules If, ElseIf, TernaryOperator, WhileLoopAlways{False,True}, DoWhileLoop, BooleanNot, BooleanAnd, BooleanOr and LogicalXor ConstantConditionRule (LastConditionVisitor, first-level statements, FunctionCallConstantConditionRule's ImpossibleCheckType ownership approximated by the type-specifying functions and assert tags) and NoopRule's match exhaustiveness. Batch 4 adds StrictComparisonOfDifferentTypesRule, ConstantLooseComparisonRule, NumberComparisonOperatorsConstantConditionRule and ImpossibleInstanceOfRule's always-true/false (types through the PHPStan type converter; instanceof.invalidExprType not yet). Batch 5 adds ImpossibleCheckType{Function,Method,StaticMethod}CallRule (the type-specifying functions decided by Mago's return type, a void function's `@phpstan-assert` against the arguments; FunctionCallConstantConditionRule's deferral narrowed to the functions ImpossibleCheckTypeHelper can decide, so `is_resource()`, `ctype_alpha()` and file functions report as constant conditions). ImpossibleInArrayHaystackFiniteTypesRule is behind featureToggles.finiteTypesInHaystack (bleeding edge) and not ported yet. Batch 6 adds MatchExpressionRule (arms narrowed by removing single values from scalar, null and enum subjects; the subjects' types come from a copy of the file with `default`-only matches, since Mago keeps a subject's last-arm type; TryCatchTypeVisitor and the `@throws` tag) and ConstantConditionInTraitRule (trait conditions collected per class context, reported once when the contexts agree; Mago types a trait's `$this` as the trait, so context-dependent conditions stay silent). Batch 7 adds ImpossibleCheckTypeHelper's in_array(), method_exists(), property_exists() and class_exists()-family answers (with what their type-specifying extensions narrow) and instanceof.invalidExprType; the treatPhpDocTypesAsCertain tip still needs native types. Batch 8 adds CallTo{Function,Method,StaticMethod,Constructor}StatementWithoutImpurePointsRule: the declarations' call dependencies (the impure points model of batch 2, keyed by PossiblyPureCallTransitivePurityResolver) travel as encoded errors resolved once every file is analysed; NewHandler's final `new` objects are approximated by local variables only ever assigned a `new`; a trait's methods declare nothing. Batch 9 (PHPStan 2.3.1) adds NullCoalesceRule through IssetCheck (`checkAdvancedIsset` from level 4; nullCoalesce.unnecessary in bleeding edge for a `null` right side): a variable's certainty and `hasExpressionType()` come from an expression hook reading Mago's block context, which also gives a `??=` target's type before the assignment; PropertyInitializationExpr follows a body's own statements (an `if` whose every branch assigns) and the constructor's, remembered unless the class serializes itself; dead_code gains a run with the whole bleedingEdge.neon. Batch 10 adds the bleeding-edge SwitchConditionRule (duplicates under PHP 8 `==`, Type::looseCompare(), the subject narrowed by the cases that leave the switch, all of a fall-through group against the subject it started with; the null, bool, 0, '' and empty-array narrowings are not ported and stop the later cases) and SortWithoutEffectRule (level 5; the treatPhpDocTypesAsCertain tip compares a parameter's native declaration, a property or offset always gets it). Held: function/callable NoDiscard (Mago callable unions keep different NoDiscard metadata, shapes lose insertion order, ConstString loses the class-string flag; reproducer tests/fixtures/rules/data/Functions/data/helper-function-discard.php) | in progress |
| analyse dead-code gaps (gaps.txt): CallTo*StatementWithoutImpurePointsRule and its collectors (transitive purity of user bodies) unported; built-in throw-type extensions (createFromFormat, DateTimeImmutable::sub); a call through a closure-typed variable (`$f()`) needs the closure's impure points; unreachable after an assert-remembered method call; unused private property/method in bug-11802 (`mixed~'x'` subtraction types); anonymous-class paths in NoopRuleTest/UnusedPrivatePropertyRuleTest exports; constant conditions: the treatPhpDocTypesAsCertain and possibly impure tips need native types, Mago type differences (closures writing $this properties, casts, static:: constants); ImpossibleCheckTypeHelper's in_array/method_exists/property_exists/class_exists cases and extensions unported; MatchExpressionRule: TypeCombinator::remove() for arrays, class strings and templates, Mago's int ranges (rand(), <=>), `$this` in traits | todo |
| analyse laravel follow-up: Larastan's model/builder forwarding (ModelForwardsCallsExtension, `__callStatic` to `Builder<static>`: `User::first()`, `User::where()`, the DB facade) and the columns of models outside the analysed files. Gaps: RelationExistenceRuleTest::testRule, NoUnnecessaryCollectionCallRuleTest::testNoFalseNegativesEloquent / testNoFalseNegativesQuery (tests/fixtures/larastan/gaps.txt) | todo |
| analyse laravel follow-up: view finder paths and hints packages register at boot (Livewire's resources/views/livewire, anonymous components' resources/views/components, `vendor::` namespaces). Pun reads resources/views only, so pinkary with checkUnusedViews + checkMissingTranslations gives 225 of PHPStan's 320 larastan.* errors: every missing one is a larastan.unusedViews of a components/* or livewire/* view, under the names those paths give it (`feed` for livewire/feed.blade.php) for a components/* or livewire/* view, none extra | todo |
| analyse laravel follow-up: config and view files in readdir order (Pun sorts; ConfigParser and ViewFileHelper follow RecursiveDirectoryIterator), and config values typed by `$scope->getType()` beyond literals, casts, `::class`, `env()` and declared function returns (larastan/config.rs) | todo |
| analyse laravel follow-up: model appends reads database/migrations only (`databaseMigrationsPath`, squashed dumps not yet) | todo |
| analyse phpdoc/operators: type-alias rules, phpdoc/invalid, invalid-phpstan-tag, the require-extends/-implements/sealed tag rules, the mixin/method/property tag rules (class, trait, trait-use), invalid-var-tag-type, invalid-param-callable-tag, the (method) assert and conditional-return tag rules, the type-mismatch rules (Incompatible{,Property,PropertyHook,ClassConstant}PhpDocTypeRule) and the 15 strict-rules `operators` (`strictRules.booleansInConditions` / `numericOperandsInArithmeticOperators`) ported, with the global `typeAliases` (phpdoc_tags.rs, require_tags.rs, class_tags.rs, var_tags.rs, callable_tags.rs, assert_tags.rs, conditional_return.rs, phpdoc_types.rs, operators.rs; a class's local type aliases are in its members' NameScope; pun_phpstan::phpdoc is a port of phpdoc-parser 2.3.3 with its exception messages). invalid-var-tag (WrongVariableNameInVarTagRule's variable-name checks, var_names.rs, with a syntactic `hasVariableType()`), GenericCallableRuleHelper (generics.rs `generic_callables`; callable types keep their templates), MethodAssertRule's inherited asserts and GenericObjectTypeCheck's / GenericAncestorsCheck's call-site variance errors (`Type::Projection`; a class's template variance from its `@template-covariant`/`-contravariant` tags) ported. VarianceCheck (`generics.variance`, `function.variance`, `method.variance`: FunctionSignatureVarianceRule, MethodSignatureVarianceRule, PropertyVarianceRule and GenericAncestorsCheck's; variance.rs) ported, with built-in classes' template variance exported into symbols.json (`variances`). Inherited method and property PHPDoc types map the ancestor's templates through the class's `@extends`/`@implements` arguments (or their defaults) (`resolveTemplateTypeInTag()`, declared.rs `ancestor_types()`). FunctionDefinitionCheck's level-2 PHPDoc class checks (`checkThisOnly` off: the classes of `@param`, `@param-out`, `@param-closure-this` and `@return` types and their class-name case, signatures.rs `phpdoc_classes()`; a `@return` type must be followed by whitespace) ported; an anonymous class's methods see the templates of the function or method it is declared in (declared.rs `enclosing_templates()`); the property hooks' PHPDoc classes (ExistingClassesInPropertyHookTypehintsRule: a get hook returns the property's `@var`, a set hook's parameter takes its own `@param` or the `@var`) and ExistingClassesInPropertiesRule's PHPDoc type classes from level 2 (ponytail: a promoted property's type from the constructor's `@param` is not read). `missingType.callable` (`checkMissingCallableSignature`, Project → Checker; MissingTypehintCheck's rules, the class-level `@method`/`@property`/`@mixin` checks and AssertRuleHelper), LocalTypeAliasesCheck's and SetPropertyHookParameterRule's missing-type checks (a trait's PHPDoc in a using class's context resolves the trait's type aliases, `self`/`static` as the class) ported. Left: VarTagTypeRuleHelper (`varTag.nativeType`/`varTag.type`/`phpstanApi.varTagAssumption`: needs native expression types), var-tag-type-mismatch (VarTagChangedExpressionTypeRule) | todo |
| analyse phpdoc follow-up (pun-phpstan resolve.rs): `value-of`/`key-of` of a template (resolved late) kept as a `value-of`/`key-of` class phpdoc_types.rs skips (`late()`); an OffsetAccessType of a template or an object (ArrayAccess) or with a non-constant offset of a constant array is not resolved (`resolve_late()`), nor checked against the template's bound in resolveOffsetAccessNode(); pure and static callables keep no purity or staticness; a constructor's own `@readonly` tag does not make its promoted properties covariant for PropertyVarianceRule; a call-site variance is not applied to the template's position (`resolveTemplateTypes()`) | todo |
| analyse phpdoc follow-up (pun-phpstan types.rs): narrow a conditional type's branches by its condition (`getNormalizedIf()`), and take the `@phpstan-template` tags over the `@psalm-template` ones (gaps.txt: IncompatiblePhpDocTypeRuleTest testBug10622/10622B, Methods ReturnTypeRuleTest testBug4605) | todo |
| analyse phpdoc follow-up: `missingType.callable` of AssertRuleHelper (and MissingTypehintCheck's other callers) needs the `checkMissingCallableSignature` parameter (off by default) read from the config (assert_tags.rs, missing_types.rs) || done |
| analyse operators follow-up (an-types): Mago's `mixed` is always implicit in convert.rs, so BooleanRuleHelper's explicit-mixed check (`Only booleans are allowed in &&, mixed given`) never reports a `mixed`-typed parameter or call | todo |
| analyse phpdoc follow-up: phpdoc_tags.rs and var_tags.rs check the doc before every statement, also those PHPStan finds unreachable (an UnreachableStatementNode is virtual); php-parser's `Nop` is read before a block's, class-like's or the file's end only (not a `case` list's or a colon-syntax block's) | todo |
| analyse phpdoc follow-up: IncompatiblePhpDocTypeRuleTest testBug13652 reports generics.* Pun extras (gaps.txt) | todo |
| analyse phpdoc follow-up (pun-phpstan): move doc::Doc::parse's approximate tag reader onto phpdoc::tags | todo |
| analyse phpdoc follow-up: var_names.rs approximates `hasVariableType()` syntactically (a write before the statement or in an enclosing loop defines a variable, also on an always-terminating path or after `unset()`; an untyped method call's arguments are not by-reference), and checks statements PHPStan finds unreachable | todo |
| analyse 3 follow-up (from 3g): names.rs hint_classes misses PHPStan's class.notFound for a `?parent` return (`self` and `static` outside a class done) | doing (independent worktree) |
| analyse 3 follow-up (from 3g): run the signatures.rs checks on anonymous-class methods beyond parameter defaults, with the anonymous-class naming follow-up below | todo |
| analyse 3 follow-up (from 3g): replace the TypehintHelper::decideType approximation (pun-phpstan is_super_type_of, Mago is_instance_of on Maybe) with a port once analyse 4 maps Mago types to PHPStan's | todo |
| analyse 3 follow-up (from 3f): describe the expression type of `instanceof.trait` beyond `$this` and `mixed` (needs analyse 4's Mago-to-PHPStan type descriptions) | todo |
| analyse 3 follow-up (from 3e): port FunctionCallParametersCheck's argument.* identifiers for attribute constructors (argument.unknown appears at level 0 in ClassAttributesRuleTest::testRule) | todo |
| analyse 3 follow-up (from 3e): `#[NoDiscard]` reads a PHPDoc return type only without a native one; port TypehintHelper::decideType for a PHPDoc subtype of a native type | todo |
| analyse 3 follow-up (from 3d): `assign.this` on a foreach target inside a list or dim fetch repeats once per loop-analysis pass in PHPStan (3 identical errors), and `catch (E $this)` is reported only when the try block can throw; both need PHPStan's loop and throw-point analysis, revisit with analyse 4 | todo |
| analyse 3 follow-up (from 3d): apply the ArrayAccess exemption of InvalidVariableAssignRule inside anonymous classes (Pun has no class metadata there) | todo |
| analyse 3 follow-up (from 3d): resolve by-reference callees for OffsetAccessWithoutDimForReadingRule beyond named functions, closure/arrow literals, $this or single-named-object method calls, static calls and new; pick the matching variant of multi-variant built-ins instead of the first | todo |
| analyse 3 follow-up (from 3c, 3f): name anonymous classes (`class@anonymous/<path>:<line>`) in DuplicateDeclarationRule, FinalPrivateMethodRule and FinalPrivateConstantRule too, with the 3b follow-up above; then lift 3f's anonymous-class skips (method.abstract, method.nonAbstract, property.callableType, class.serializable) and the MissingMethodImplementationRuleTest::testRule gap | todo |
| analyse parser semantic checks: complete php-parser's remaining semantic validation before name resolution and callbacks | doing (accepted try, property-hook, nested-namespace, attributed constants, namespace postprocessing, pipe parentheses, reference-to-new, for/echo commas and empty-array checks pass combined public, fixture and coverage gates; general recovered grammar still needs provenance) |
| analyse pure declarations: complete PureFunctionRule/PureMethodRule through FunctionPurityCheck beyond the integrated conservative void.pure branch | doing (combined public, fixture, CLI and coverage gates pass; 51 level-fixture and six coverage-fixture locations remain explicit unsupported diagnostics; complete mappings stay inactive) |
| analyse 3 follow-up (from 3b): pass the analysed paths and cwd into `Checks` so ConstructorReturnTypeRule names anonymous classes as FuzzyRelativePathHelper does (`class@anonymous/<path>:<line>`) | todo |
| analyse 3 follow-up (from 3b): examples/analyse in a debug build overflows the default thread stack on laravel-framework (ran with RUST_MIN_STACK=268435456); find the deep recursion or raise the stack in the corpus script | todo |
| analyse 3 follow-up (from 3b): re-export the Cast/VoidCastRule fixtures with a PHP 8.5 oracle to confirm cast.void against a live run (the 8.4 oracle cannot parse `(void)`) | todo |
| analyse 3 follow-up: nested abstract-trait prototype messages can name the source trait rather than its importing trait; competing overriding prototypes use lexical rather than source order | todo |
| analyse 3 follow-up: apply a trait alias's visibility (`foo as protected`) to the walked trait method | todo |
| analyse 4-5: type inference; levels 1-10 as the five presets; strict-rules, deprecation rules. Level 6 missingType.* and level 2 Generics rules are ported (missing_types.rs, declared.rs, doc.rs, generics.rs). Not yet: call-site variance (`generics.callSiteVariance*`, needs variance in phpdoc::Node::Generic), VarianceCheck (`generics.variance`), `generics.unresolvable`, CrossCheckInterfacesHelper (`generics.interfaceConflict`), the gap `2 Generics/ClassAncestorsRuleTest::testRuleExtends`. Corpus: pinkary, pest, larastan exact for generics.* and missingType; laravel-framework differs only in anonymous-class names | todo |
| analyse types (levels 1-9, `analyse/types/*`): ported MissingReturnRule (return.missing/never, the php-parser trailing-comment `Nop` end statement, exhaustive switch over enum cases and literals), RegularExpressionPatternRule, Echo/Print/InvalidPartOfEncapsedString/InvalidCast/InvalidUnaryOperation/InvalidComparisonOperation, UsageOfVoidMatchExpression, the four IncompatibleDefaultParameterType rules, Function/Method/Closure/ArrowFunction ReturnTypeRule (get/set hooks too). Level 3: ThrowExprType, VariableCloning, IterableInForeach, UnpackIterableInArray, ArrayUnpacking, InvalidKeyInArrayDimFetch, the three Yield rules, DefaultValueTypesAssignedToProperties (types/values.rs, yields.rs, property_default.rs; findTypeToCheck's class.notFound errors with class_exists() guards), TypesAssignedToProperties, ReadOnlyPropertyAssign(Ref), ReadOnlyByPhpDocPropertyAssign(Ref) (types/properties.rs: PropertyReflectionFinder, `@property` over a native property by hierarchy distance, @readonly/@immutable tags); level 2: ValueAssignedToClassConstant (constant_value.rs), WritingToReadOnlyProperties beyond `$this`, InvalidBinaryOperation (binary_op.rs: the ErrorType results of InitializerExprTypeResolver's arithmetic, bitwise, concat and exponentiate); level 3 also OffsetAccessAssignment, OffsetAccessAssignOp, OffsetAccessValueAssignment (offsets.rs: setOffsetValueType()'s ErrorType, `offsetSet()` from PHPStan's stubs for built-in classes; Mago's `never` argument of an unresolved template is its bound); level 4: DeadForeach, the TooWide function/method/closure/arrow-function return and property rules (too_wide.rs: the union `.unusedType` checks; `reportTooWideBool` and `reportNestedTooWideType` are off by default). NullsafePropertyFetch/NullsafeMethodCall (`nullsafe.neverNull`, nullsafe.rs: the treatPhpDocTypesAsCertain tip from a user method's native return type), the three ThrowsVoid*WithExplicitThrowPoint rules (throws_void.rs: throw, user function/method/constructor `@throws` points minus the try's catches), InvalidIncDecOperation `.type` (inc_dec.rs) and CatchWithUnthrownException's `catch.alreadyCaught`. Shared: rules/types/flow.rs (StatementResult, ExecutionEndNode), convert.rs (Mago type -> pun_phpstan Type), level.rs (RuleLevelHelper findTypeToCheck/accepts), pun-phpstan types/accepts.rs (`Type::accepts()` in each type's order, checked pair by pair against PHPStan in tests/fixtures/type-accepts.json), casts.rs (toString/toNumber/... ErrorType, type-casts.json), verbosity.rs (getRecommendedLevelByType). Left: accepts reasons tips, template variance (the symbols carry none: built-in covariant templates count as invariant), a constant array's keys against another constant array in accepts, closure/arrow `@param`/`@return` PHPDoc, property writes through offsets and list destructuring, set-hook parameter types, the level 3 param-out rules, built-in functions' and dynamic throw types (throws-void), `catch.neverThrown` (needs implicit throw points), the TooWide param-out rules (they need a variable's type at the function's end), TooWide property writes in used traits, list elements, `++`/`--`, foreach and by-reference arguments (skipped), IssetCheck's `?->` messages (the native type of non-method holders counts as the PHPDoc one), the rest of level 4 and levels 5-9; the level gaps list the Mago inference differences | todo |
| analyse bug: anonymous class names carry the scratch path (`Foo@anonymous/scratch/laravel-framework/src/...`); PHPStan writes `Foo@anonymous/Illuminate/...` (68 missingType differences on laravel-framework) | todo |
| analyse follow-up: crates/pun-phpstan/data/symbols.json is the Linux export (constants such as MSG_* differ on macOS); the method `doc`/`propertyDocs` data was merged in from a macOS run, so re-run `scripts/export-phpstan-symbols.php` on the Linux server to refresh it and its php85 entries | todo |
| analyse 4-5: all 385 analyse rules native (one PHPStan Rule class each, rules.md §7) | todo |
| analyse types2 (levels 2-9 on typed receivers, an-types2): ported FunctionCallParametersCheck's level-5 checks (`argument.type`, `argument.byRef`, `argument.unpackNonIterable`; types/arguments.rs) with selectFromTypes over the variants (params.rs `winning()`), ParameterCastableToString/Implode/Sort, RandomIntParameters, ArrayFilter, ArrayValues (castable.rs), MethodCallCheck/StaticMethodCallCheck on any receiver (`method.nonObject/notFound`, `staticMethod.*`; receivers.rs) and AccessPropertiesCheck and AccessStaticPropertiesCheck on any receiver (`property.*`, `staticProperty.*`, with `@mixin` and `@phpstan-require-extends` members; properties.rs), through findTypeToCheck at each level; CallCallablesRule (`callable.*`, pipes included), CallUserFuncRule, RegularExpressionQuotingRule (`argument.invalidPregQuote`) and FunctionDefinitionCheck's `@param`/`@return` class.notFound (level 2). GenericParametersAcceptorResolver (templates inferred from the arguments, types/templates.rs in pun-phpstan) and selectFromArgs' callback parameters of array_map/array_filter/array_walk/array_find*/implode (selector.rs), and accepts-reason tips on argument.type (types/reasons.rs in pun-phpstan, `Levels::accepts_tip()`), `argument.templateType` (with parameter defaults, unpassed variadics and conditional return types in the inference), curl_setopt()/curl_setopt_array() option types (curl-options.json exported from the oracle), the Closure::bind()/bindTo() `@param-closure-this` override, a bound closure's class scope (bind_scope.rs) and PHPStan's dynamic return types as Mago providers for the rules' inference (return_types.rs: functionMap's `__benevolent<…>` returns such as curl_init(), ClosureBind(To)DynamicReturnTypeExtension). Left (level gaps, "an-types2" block): closure parameters narrowed by the callable they are passed to (needs Mago's analyser to intersect a typed closure parameter with the expected callable's, as MutatingScope::enterAnonymousFunction() does), Mago's constant-array key order, NewIsAlwaysFinal object types (Mago's named objects carry no exactness, and a variable's type loses its `new` origin), `Foo::class` of an unknown class as a class string, DateTimeInstantiationRule (needs PHP's timelib date parser: port or vendor, Pushpak to decide), unresolved `@mixin T`, dynamic member names, asymmetric visibility on writes, and Mago inference differences (no types after an undefined property, list vs array, narrowing) | todo |
| analyse (from mapping): re-link `fixed_by` from analyse/types/missing-param-type, missing-property-type and missing-return-type to the split lint type rules where one fits | todo |
| analyse (from mapping): consider dedicated home rules for identifiers that many classes emit (class.notFound, missingType.iterableValue), so migrated ignores read naturally | todo |
| analyse (from mapping): revisit the Symfony and phpstanApi exclusions if the scope widens | todo |
| analyse 6: Larastan with the seven improvements | todo |
| analyse 6 follow-up: aliases from dynamic config, provider/bootstrap code, programmatic AliasLoader changes, custom application paths and real-time facades; preserve cold case-sensitive lookup order and unloaded real-class collisions instead of graph-wide alias activation and real-declaration precedence | todo |
| analyse 6 follow-up: Larastan knowledge Pinkary needs first: auth() return types (53 Mago issues), Http/facade responses (7), container/Storage (6), plus model accessors and relations | todo |
| analyse 8: result cache | todo |
| lint (from config, migrate 4-5 follow-ups): `pun migrate` writes Rector configurations as rule options; the lint rules should read them once ported (analyse reads its [phpstan-parameters] options through `Analyse::parameters`) | todo |
| analyse follow-up (from config): a stub of an analysed (Host) class: Mago leaves a user-defined class unpatched (`apply_patches_pass`), PHPStan's stub wins; stubs of vendor classes apply alike | todo |
| analyse follow-up (from config): `pun check` (run.rs) drops `Analysed.general` (unmatched `[[analyse.ignore]]` entries without a file path) and doesn't stop on "No rules detected"; `pun analyse` reports both | todo |
| analyse follow-up (from config): bleedingEdge's behaviour toggles (`internalTag`, `checkGenericIterableClasses`, ...) stay off, as migrate reports; only the toggles that gate a rule class follow the rule. Pinkary at `level: max` with Larastan + bleedingEdge, migrated: PHPStan 1 error (array.invalidKey, not ported), Pun 1 (`Blade::render()` class.notFound, facade aliases of analyse 6). With five `ignoreErrors` entries added and migrated (identifier+path, message+path+count, unmatched message+path, rawMessage+paths, bare pattern) the JSON reports are byte-identical except where those two residuals move an ignore (`ignore.unmatched` vs `ignore.count`) | todo |
| analyse follow-up (corpus comparison 2026-11-01, PHPStan level 1 + `checkUninitializedProperties` against the `basic` preset): `(fn () => $this->x())->call($object)` and `(function () {...})->call($object)` keep the outer `$this`; PHPStan binds it to the argument. laravel-pint FixCode.php:64 (2 method.notFound, plus the unmatched ignore at 108), laravel-framework InteractsWithTestCaseLifecycle.php:294-295 (`Undefined variable: $this`) | todo |
| analyse follow-up (same comparison): `isset($last) && $x < $last` in a loop, with `$last` assigned later in the body, reports "might not be defined". The first loop pass reads `$last` as undefined and `observe_direct` merges the passes into Maybe; PHPStan reports from the converged scope. laravel-framework SortedMiddleware.php:48, CanBeOneOfMany.php:107-108 | todo |
| analyse follow-up (same comparison): `if (($d ?? false) === false) { return; }` does not narrow `$d` to defined afterwards (Encrypter.php:204). `$tag ?? ''` after `openssl_encrypt(..., $tag)` (Encrypter.php:118), `$config['transport'] ?? ...` (MailManager.php:178), `isset($session)` on `app('session')` (helpers.php:404) and a nested `??` offset (CloudBootstrapper.php:170) report nullCoalesce/isset errors that PHPStan does not | todo |
| analyse follow-up (same comparison): level 1's magic-member reporting on classes with `__call`/`__get` (PHPStan reports 16 method.notFound and 10 property.notFound on laravel-framework, e.g. MustVerifyEmail.php in context of Foundation\Auth\User, and staticMethod.notFound that ProcessDriver.php:42/90 ignore), `unset.possiblyHookedProperty` (Route.php:1568), and Pun's extra `method.nameCase` (MySqlGrammar::compileFullText) | todo |

**Gate:** the same errors as PHPStan + Larastan (file, line, identifier) on the corpus, at every preset.

### Step 4. `pun check` in one pass (plan/commands.md Build)

| Work | Status |
| --- | --- |
| check 1-5: one parse per file, analyse joins, `--fix` loop | todo |

Each command joins as soon as its track has it, so this step closes when the last one joins.
**Gate:** `pun check --fix` on a pint.json project equals `pint`, and on every corpus project it equals running the
three commands one after the other.

### Step 5. 1.0 (plan/foundations.md)

Docs pages for every command and rule, `pun explain`, the generated "Coming from" pages (plan/docs.md), AGENTS.md updated for the new
output, name checks, signing and publishing (Pushpak: after Step 4).

| Work | Status |
| --- | --- |
| docs 6: CLI rule-page links (needs the public docs origin; the plan settles the path, not the hostname) | blocked (needs Pushpak) |
| docs content: 385 analyse rules have no summary; no rule has a why or examples. Add prose and oracle-backed examples in mapping.toml in later batches | todo |
| release notices: the Zend sorting port requires the acknowledgment in THIRD_PARTY_NOTICES.md and the Zend Engine acknowledgment in advertising materials that mention its features or use (Zend Engine License clauses 5 and 6); verify those materials before publication | todo |

### Foundations, alongside every step (plan/foundations.md)

| Work | Status |
| --- | --- |
| CI: get CI on main green, then re-enable it (disabled on GitHub 2026-10-18 by Pushpak: every agent push was billing a run; `gh workflow enable CI`) | todo |
| tests: crates/pun-phpstan/tests/php.rs `php_helpers` fails on macOS on two error offsets (`/a)/` gives 2, PHP 1; `/(?<1a>x)/` gives 4, PHP 3); the linked pcre2 likely differs from PHP's PCRE2 | todo |
| pun-phpstan: clippy collapsible_if at src/schema.rs:754 | todo |
| pun-harness: the comment in src/main.rs about the starter's `preset = "recommended"` is stale (migrate no longer writes it) | todo |
| CI: clippy gate (`-D warnings`); clippy is at 0 warnings | todo |
| CI: CLI polish item 7 (`script` check in CI, skip message in tests/migrate.rs) | todo (stopped by Pushpak) |
| harness: lint and analyse modes; pun-harness dev-only; fixtures into pun-fixers | todo |
| scripts/fuzz-parity.sh: drop the `PUN_PINT` passthrough and its comment (delegate mode removed) | todo |
| reports/scoreboard.md: regenerate or mark obsolete the 'Delegate mode' section and `--native-only` timings | todo |
| design/components/DocsPage/preview.html: remove the 'Needs PHP for those rules' / `--native-only` callout | todo |
| oracle: the local PHP (Herd ini) runs with pcre.jit=Off, so preg limit errors read 'Recursion limit exhausted' where PHP's default (and Pun) give 'JIT stack limit exhausted'; run preg oracles with `-d pcre.jit=1` | todo |
| fixers: binary_operator_spaces with an array in `operators` prints PHP's 'Warning: Array to string conversion' before the configure error upstream; Pun prints only the error | todo |
| audit fixers-messages follow-up: run scripts/cli-parity.sh on a bad fixer config to confirm the reworded configure errors end to end | todo |
| perf (from bench): analyse is slower than a warm PHPStan result cache (pun 0.5-1.8 s vs 0.4-0.8 s) and 1.5-2.2x slower than Mago's analyser; targets for the perf rounds (reports/scoreboard.md "Analyse benchmark") | todo |
| coverage: the php_block_formatting.rs:221 exclusion in reports/COVERAGE.md is keyed by line number and moves with any edit above it; key it by something stable | todo |
| audit item 46 follow-up: Pun/use_specific_expectations (like its sibling Pest rules) now skips `namespace\expect(..)`, which Mago 1.57.0 fixes in the global namespace; decide whether all Pest rules should accept it | todo |
| fixers: switch_continue_to_break decodes `continue 0X1F;` as octal because it checks only lowercase `0x`/`0b`; this matches upstream SwitchContinueToBreakFixer.php, so keep it unless upstream changes | todo |
| audit item 51 follow-up: FormatPreset enum for `Config.preset` (read as a string in pun-cli/src/run.rs and main.rs) and FixSafety via pun_problem::Safety for `RuleMeta.fix == "safe"` in pun-cli/src/report.rs; waits for the platform/format tracks that own those files | todo |
| audit item 55 follow-up: switch to_fixer's `{at}: ` prefix and constraint::lowest's 'invalid constraint' prefix to `.with_context` once pun-cli/src/watch.rs prints `{e:#}` | todo |
| tests: crates/pun-cli/tests/cli.rs:327 and output.rs:816 fake read errors with mode 000 like audit item 34 did; assert the error only when std::fs::read really fails | todo |
| tests: crates/pun-tokens/tests/api.rs:317 swaps the process panic hook (as docblock tests did, audit item 80); use catch_unwind alone | todo |
| perf (from mapping): rerun scripts/bench.py after the perf rounds and keep iterating on parity and benchmarks | todo |
| perf lint 1 follow-up: scripts/bench.py lint and analyse modes report time from a run without the 20 ms RSS polling (or poll RSS in a separate run), so runs under 100 ms show real differences | todo |
| audit php-compat follow-up: an invalid PHP regex in a pint.json notPath/notName (unknown modifier, missing end delimiter) is now a run error with PHP's message, as Pint fails; mention it on the docs config page | todo |
| pun-phpstan: with the `n` modifier `php_compat::preg_compile` prepends `(?n)`, so the error offset of `/(a)\1/n` is 5 where PHP says 4; set PCRE2_NO_AUTO_CAPTURE as the fixers' raw compile does | todo |
| tests: on macOS PHP rejects a non-ASCII regex delimiter such as `§` as alphanumeric (locale isalnum) while php_compat::preg_parse reports an unknown modifier; check glibc PHP and add a case if they agree | todo |
| perf lint 1 follow-up: if the pun-lint tree walk grows with more rules, skip leaf node kinds no rule targets (about 28% of nodes: DirectVariable, Keyword, LocalIdentifier, Terminator, LiteralString); under 3% today | todo |
| perf lint 2 follow-up: peak memory is Mago's parse arena (about 41x source size; a 462 KB file takes a 26 MB arena, 39k allocations of 376 bytes); cutting it needs an upstream change in Mago, not pun-lint | todo |
| perf lint 2 follow-up: opening files is the largest system cost (about 25% of samples) and does not scale past 4 threads on macOS; openat through a held directory handle did not help | todo |
| perf lint 3 follow-up: when the Mago crates are bumped, rerun crates/pun-lint/tests/walk.rs; if Mago adds a node type, add a hook line in the walker macro and a fixture snippet (the test's count of 221 node types flags a missing one) | todo |
| perf lint 3 follow-up: opening files is now 33% of lint samples (about 11 us per file); in `pun check`, read each file once and share the source between format, lint and analyse instead of opening it once per pass | todo |
| perf lint 4 follow-up: in `pun check`, read each file once for format, lint and analyse; `open` is now the critical path of lint (82% of a reader thread's time) | todo |
| perf lint 4 follow-up: if pun-cli gives pun-lint a mago_database File per source, call parse_file_borrowed_with_settings on it directly and drop the unsafe lifetime extension in `Linter::check` | todo |
| perf lint 4 follow-up: check whether the 10.5 us per-file `open` (against 1 us for `stat`) comes from the com.apple.provenance xattr on the corpus files; if so it is an environment cost end users would not pay | todo |
| perf lint 5 follow-up: the tree walk is the only engine lever left (removing it cuts wall 13%, but Pun's hooks cost about 2 ns per node and skipping leaf types gained only 2.2% single-thread); a larger win needs a cheaper tree walk in Mago, or rules that do not need a full ancestor stack | todo |
| perf lint 5 follow-up: Mago's parser is about 80% of lint check CPU; further gains need upstream Mago parser work | todo |
| perf lint 5 follow-up: macOS `open` (10.5 us on one thread, 15 us on three) is an environment floor; com.apple.provenance cannot be removed with `xattr -d`, so whether it causes the cost is still open | todo |
| perf lint 6 follow-up: ask upstream Mago to make mago-database's `notify` dependency optional (or use the kqueue backend); its FSEvents backend links CoreFoundation and CoreServices, about 1.0 ms and 4.1 MB per process (15% of pinkary's lint time); pun-cli's `--watch` would then need a notify backend that does not link CF, or load it lazily | todo |
| perf analyse 1 follow-up: scripts/bench.py polls `ps -A` every 20 ms, which slows the measured process (pinkary 0.53-0.86 s in the bench vs 0.48-0.60 s under /usr/bin/time); a waiter thread with wait4's ru_maxrss would be cheaper and exact | todo |
| perf analyse 2 follow-up: pun-cli: consider mimalloc as the global allocator once `pun analyse` lands; in the analyse example it gave wall -4 to -5% and rss -6% on pinkary and laravel-framework, +7% rss on laravel-pint | todo |
| perf analyse 2 follow-up: bench.py 3-run analyse medians vary 5-10% between calls; the wait4 ru_maxrss waiter (perf analyse 1 row above) would cut the noise | todo |
| perf analyse 2 follow-up: Mago's populate (15-20% of analyse wall) and its parser memory are upstream Mago costs; raise them upstream rather than in pun-analyse | todo |
| perf analyse 3 follow-up: scan PHPStan's API stub (phpstan-api.php, about 60 ms serial on larastan and pest) off the critical path; `rayon::join` beside the scan rounds gave -20% wall but 2 of 14 runs stalled for 600-1000 s, so find the cause (Mago interner or rayon) before retrying | todo |
| perf analyse 3 follow-up: walk psr-4 and psr-0 directories lazily, reading only the directories a class lookup needs (the directory walk is about 18% of samples; about half the directories hold a loaded file) | todo |
| perf analyse 3 follow-up: a lexer-aware name scan (code names plus docblocks only) would load about 8% fewer files on Pinkary; skipping vendor method bodies would load about 17% fewer, but Mago infers assertions from single-return bodies, so it needs a parity plan | todo |
| audit item 82 follow-up: docblock/tests.rs `type_expression_parse_invalid` still checks the message with `contains`; item 15 (var_export) has landed, so assert the full message with oracle values | todo |
| audit item 39 follow-up: docblock Annotation's types_content/types caches are still RefCell clones per read; a OnceCell needs set_types/remove to take &mut self, which DocBlock's OnceCell<Vec<Annotation>> does not allow | todo |
| audit item 41 follow-up: other patterns built from file content still go through Preg's never-evicting cache (no_unused_imports' comment pattern, method_chaining_indentation); move them to the `_uncached` entry points | todo |
| coverage: measure punphp (pun-cli) on Linux after audit item 33 (ef7c179); migrate::announce's decline branch is reached only by the Linux-only pty tests | todo |
| audit-hygiene follow-up (item 68): move crates/pun-lint/Cargo.toml's homepage, keywords and categories to `.workspace = true` (left for the lint track, which owns the file) | todo |
| audit-hygiene follow-up (item 66): pun-lint's examples repeat their file walkers; share one helper (left for the lint track) | todo |
| audit-hygiene follow-up (item 87): `unreachable_pub` is still open in pun-cli run.rs, report.rs, pint_error.rs, fix_report.rs, watch.rs, main.rs, console.rs, render.rs, in pun-analyse/src/rules and in pun-lint (files owned by other tracks); without the item 28 lint gate the narrowing can regress | todo |
| clippy: pun-cli tests/output.rs:467 (`iter().copied().collect()`) warns on main (the pun-analyse warnings were fixed in 52ec4f2) | todo |
| oracle 2.3.1 follow-up: port PHPStan 2.3's new rules, all unported in mapping.toml: SwitchConditionRule (switch.alwaysFalse/alwaysTrue/duplicateCase), UnusedVariableRule (assign.redundant), UnusedFunctionParametersRule and UnusedMethodParametersRule (bleeding edge, level 4), SortWithoutEffectRule (bleeding edge, level 5), ReturnTypeAfterFinallyRule (return.byRefFinally, level 3), PurePropertyHookRule (level 2) | todo |
| oracle 2.3.1 follow-up: PHPStan 2.3 narrows the scope's PHP version by `PHP_VERSION_ID` conditions; the rules reading `$scope->getPhpVersion()` follow the `PHP_VERSION_ID` guards (into functions, classes and traits used in the branch; printf's `h`/`H` too); left: a configured `phpVersion` min/max range (Pun keeps one version) | done |
| oracle 2.3.1 follow-up: UnusedClosureUsesRule and UnusedConstructorParametersRule moved to VariableWritesNode dataflow (closure.unusedUseFlow, constructor.unusedParameterFlow: a use or parameter overwritten before any read is unused); Pun keeps the 2.2 name check (levels/gaps.txt) | todo |
| oracle 2.3.1 follow-up: PHPStan 2.3's scope and impure points: a `for` loop whose body always exits checks its condition once (tests/dead_code.rs GAPS), an expression that can throw an Error (`/`, `%`, sprintf(), strpos(), array_combine()) is no noop, PHPDoc-certain tips, nullsafe short-circuits that terminate; ClassConstantAccessType (`T::CONFIG_*` is deferred, no unresolvable type); the inferred types in return.type messages (levels/gaps.txt) | todo |
| oracle 2.3.1 follow-up: 2.3's template-type descriptions (CallMethodsRuleTest::testArrayShapeTemplateKey, testGenericsEmptyArray, OverridingMethodRuleTest::testBug9615) | todo |
| oracle 2.3.1 follow-up: PrintfFormatParser's placeholder count, FilterVarRule's filter_input()/filter_var_array()/filter_input_array() checks, the `*.nameNotString` checks (bleeding edge checkNonStringableDynamicAccess), DynamicClassConstantFetchRule's nameType from level 2, and MethodSignatureRule's purity overrides (bleeding edge reportMethodPurityOverride: method.impure, method.impureOverridePureUnlessCallable, method.impureOverridePureUnlessParameterPassed; the rule's PHPDoc return and parameter type checks stay unported); left: variable.nameNotString (DefinedVariableRule unported), explicit `mixed` names at level 9 (Mago's mixed has no explicit flag) | done |
| oracle 2.3.1 follow-up: pun-phpstan's 2.3 type gaps (tests/types.rs SIGNATURE_GAPS, PROBE_GAPS makeList intersections and `array\|static` template probes, SUPER_GAPS) and corpus/larastan's config (registers a service class the oracle's Larastan v3.10.0 lacks, skipped in tests/config.rs) | todo |
| oracle 2.3.1 follow-up (after the main merge): the 2.3.1 DynamicFunctionThrowTypeExtensions (GetClass, Printf, ArrayChunk, MinMax, Round, StrSplit, TriggerError, Unserialize, FilterFunctions) as explicit throw points in dead_code/noop.rs; 2.3.1's new regression tests for main's typed receiver/argument/property/offset/impossible-check rules; var_names' `while (true)` loop scope (levels/gaps.txt last three blocks, tests/coverage.rs GAPS) | todo |
| later: LSP; install, run, test; native Blade; extension system | after 1.0 |
| later: analyse 7, Pest and PHPUnit typing (analyse.md §9) | after 1.0 |
| audit item 17 follow-up: add property.trait to IDENTIFIERS, take identifiers as &'static str (no format! in names.rs), assert in tests/rules.rs that every reported identifier is listed, drop testPromotedProperties from gaps.txt; waits for the rules/* claim | todo |
| oracle: .oracle/phpstan's Composer project has symfony/polyfill-php85, so PHPStan run from it knows array_first() and friends at phpVersion 80400; take built-in-symbol expectations from the standalone phar outside a Composer project | todo |

## 4. Open questions

1. **After 1.0.** Are `pun install`, `pun run` and `pun test` (the archived plan's phases 5-7) part of 1.0 or after it?
2. **`--remove-tools` in migrate.** Should migrate offer to remove Pint, PHPStan and Rector from composer.json? The
   default is no: the future `pun-migrate` agent skill does workflow changes.
