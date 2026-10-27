# Pun plan

The one file for where Pun stands: how this folder works, every decision, the build order with its status, and the
open questions. Rewritten 2026-10-17 after walking through every command with Pushpak; it replaces README.md,
DECISIONS.md, TASKS.md and REQUESTS.md (their history is in git). Finished work is not listed; `git log` has it.

## 0. Resume here (2026-10-27)

One lead agent works on `main` and spawns subagents for independent slices in worktrees (AGENTS.md "How work runs").
Pushpak resumed all commands on 2026-10-25. CI stays disabled.

- **Counts** (local mapping.toml `ported = true`): format 168 of 168, lint 514 of 703, analyse 299 of 385.
  Pending: format 0, lint 189, analyse 86 (275 total). Ported rules still have the explicit parity gaps below.
- **Latest verified batch:** all 393 combined partitions pass: 1,315 rule cases, 2,787 level cases, 549
  analyser public/library tests, 431 lint tests and all 78 owning PHPStan tests. Per-value object reflection,
  foreach and standalone-block pollution, raw quoted PHPDoc bytes, normalized numeric shape keys, optional
  list metadata and failed-offset coalescing pass together without changing upstream expectations.
  Source-owned loop and foreach corrections resolve all three regressions found by the first combined run:
  the lost loop-condition upper bound in `testBug11506`, an extra empty-array template alternative and
  incorrect certainty after a terminating isset guard. Both 246-case Functions partitions pass.
  Source-matched coverage is analyse 42,846/42,846, lint 39,270/39,270 and PHPStan 10,557/10,557 after
  existing exclusions. All 874 changed executable lines are covered without new exclusions. Three verified
  obsolete level markers are removed; 653 level gaps and 16 rule gaps remain, so this is not full parity.
  All 51 analyse, lint, migrate and init CLI tests and all 20 Pint CLI comparisons pass. Pinkary formatting
  matches all 799 files; its absent-dependency analyse errors are not analyse/check parity evidence.
  Fresh byte, raw-comment, block, list and foreach-entry oracle captures are byte-identical.
  The docs build passes on 1,446 pages, with all 18 rendered contract checks passing.
  The coordinator reached 30 minutes after 340 passing partitions; the remaining 53 pass against the same
  frozen sources and executables. Original interruption logs are retained. Normal coverage export succeeds;
  all 18 zero-hash warnings name unchanged inline flag methods with positive matching profile records.
- **Next isolated work:** shared real-edge joins still have eight strict rollout mismatches, including
  missing post-join null-context specifications. The wide-integer literal correction is ready privately:
  128 tests pass, including 45 diagnostic and 171 literal records, with 41/41 changed lines covered.
  Last-declaration selection for duplicate class-like constants is assigned independently. CI stays disabled.
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
- **PHPStan parity metric:** context/reference/phpstan-identifiers.txt: the 947 identifiers phpstan.org/error-identifiers lists for phpstan/phpstan-src 2.3.x (latest release 2.3.1); `pun_analyse::reports()` recognizes 690 (2026-10-26), checked against the freshly compiled public API. This is identifier coverage, not full behavioral parity. Extensions are tracked apart: phpstan-strict-rules-identifiers.txt (29 of 53) and phpstan-deprecation-rules-identifiers.txt (72 of 120). The oracle is PHPStan 2.3.1 (wt-oracle-23).
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
- **Priorities, in order:** (1) done (the repaired combined level, rule and coverage gate passes with documented older gaps); (2) done (lint vs Rector: `scripts/lint-vs-rector.py`, residuals in §3); (3) done (lint declaration-order regressions and source-matched coverage); (4) doing: analyse level 0, then the
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
| lint 7: all 703 lint rules native, in usage order. 514 of 703 mapping.toml lint rules ported (count: `python3 -c` over `[rules.lint]` `ported`). Next, most-used groups first: types n-z, style n-z, dead-code n-z, the rest of `laravel` (65 of 79 rector-laravel mappings, most need types), then whatever group has the most unported rules | todo |
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
| lint 7 (needs Pushpak): `style/prefer-class-constant` (PHP-CS-Fixer `class_keyword`) needs the class to exist in the fixer's PHP process. Option A (closest to upstream, picked): leave it unported. Option B: port it with PHP's built-in classes only | needs Pushpak |
| lint 7 follow-up: `php-8.5/no-null-array-key` (ArrayKeyExistsNullToEmptyStringRector) needs the key's type; port it once analyse's types reach pun-lint (Build 8) | todo |
| lint 7 follow-up (from batch 2): move crates/pun-lint/src/rules/rector_code_quality.rs's shared helpers (precedence table, in_parent, statements_edit, inverse_node/de_morgan, call_to/args/is_constant/equal) into `crate::syntax` for later batches (syntax.rs is owned by lint 1-2) | todo |
| lint 7 follow-up (from batch 2): `lint/complexity/prefer-null-coalesce`'s mapping summary describes more than ArrayKeyExistsTernaryThenValueToCoalescing (isset ternaries, if/return chains, foreach lookups); check whether more upstream rules belong to it | todo |
| lint 7 follow-up (from batch 2): on a colon-body `if (...): ... endif;` Rector reprints the `if` with braces when one statement becomes two, Pun keeps the colon syntax | todo |
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
| analyse level 0 named arguments: audit the complete MethodCallWithPossiblyRenamedNamedArgumentRule contract before activating its mapping | doing (analysed call/override collectors, provenance, cross-file joins and template-offset corrections pass all 393 combined partitions and source-matched coverage; whole-rule activation remains separate) |
| analyse level 0 arrays: complete DuplicateKeysInLiteralArraysRule beyond the verified scalar/constant keys, sealed unpacking and byte-exact literal keys; identifier boundaries still use UTF-8 text | doing (shared quoted-key decoding, byte-based tokenization and comment ingestion pass the complete combined gate and coverage; identifier handling and whole-rule activation remain incomplete) |
| analyse wide integer literals: preserve overflowing token magnitude through shared syntax lowering before key conversion and copied-value inference | doing (private correction matches 45 complete PHPStan diagnostics and 171 parser literal records; identical public tests fail before and pass after; 128 parser, analyser and lint controls pass, with 41/41 changed lines covered; combined-main integration remains) |
| analyse duplicate class-like constant selection: retain the last immediate declaration's complete metadata without changing independent duplicate diagnostics or parent/trait precedence | doing (independent source owner assigned; fresh PHPStan reflection probes confirm last-declaration values and spans for classes and traits; public before/after verification remains) |
| analyse level 0 variables: port DefinedVariableRule and CompactVariablesRule on shared variable-certainty observations, including assign/undefined-allowed contexts, dynamic names and the CLI/maybe-undefined switches | doing (nested dynamic-name reads, stable array-offset filtering, literal string-byte offsets and scope-pollution settings pass the complete combined gate and coverage; general scope joins and compact's independent selector decision remain separate; no whole-rule activation) |
| analyse shared scope joins: extend PHPStan's array-preserving holder unions and branch-only offset removal to every applicable join | doing (foreach gate repairs integrated separately; isolated rollout retains eight strict matrix mismatches, including OR/assert null-context specifications and bounded-template replacement; post-join specification plumbing is assigned with artifacts, invocation and expression-statement ownership; foreach projection and loop generalization stay separate; PHPStan uses one ScopeOps contract across genuine branch joins) |
| analyse level 0 unset: port UnsetRule's readonly and hooked-property checks, recursive offset checks and definitely-undefined local checks through shared reflection, types and certainty | doing (27 integrated public tests and ten exported cases pass; required-interface native reflection and annotation writable types are integrated; fresh captures cover the bounded reflection contracts; complete rule activation and combined coverage remain) |
| analyse level 0 attributes: complete the twelve existing attribute-rule contracts against AttributesCheck, including constructor types and version/named-argument controls, before activating their mappings | doing (explicit aliases, parenthesized flags and shared trait-constructor reflection integrated; isolated reflection controls match 9/41/45 oracle diagnostics at levels 0/5/6; conditional/coalesce initializer inference remains isolated; combined gates running; mapping stays unported) |
| analyse shared class names: carry imported-name and internal-class case options through the shared ClassCaseSensitivityCheck, preserving explicit aliases and written-name distinctions | doing (isolated source port against fresh PHPStan controls; qualified special-name parser validation and source-only class_alias provenance remain separate attribute boundaries) |
| analyse compact rule selection (needs Pushpak): variable.undefined has one canonical Pun rule, so compact findings cannot be independently enabled, disabled or ignored by analyse/undefined/compact-variable | needs Pushpak (default: preserve the existing canonical identifier decision; keep compact unported while its extraction and public selector boundaries remain) |
| analyse initializer dependency cycles: extend canonical resolution beyond the currently supported global, ordinary-class and trait dependencies | todo |
| analyse dynamic initializer selectors: preserve PHPStan's mixed fallback for expression constant names such as `A::{'V'}`, separately from ordinary expression flow | todo |
| analyse shared string inference: preserve PHPStan's refinements through casing-union widening and unknown nonempty concatenation | doing (concat matches all 29 fresh PHPStan observations; union retention matches all 52 and the numeric accessory regression matches all 17; all 57 combined object, native, casing and repeat tests pass; implied nonemptiness remains semantically nonempty without inventing an independent accessory; existing expectations and negative controls are unchanged; combined gates running) |
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
| analyse 3 follow-up (from 3b): skip statements after a terminating one in the rule walk, as PHPStan does (unreachable statements; later never-returning calls from inference) | todo |
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
