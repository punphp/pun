# Plan: analyse (`pun analyse`)

Status: **walked through with Pushpak** (2026-10-17); answers in "Decisions". This file is the plan for the analyser:
how it works today, the decided design, and the build list (§6). Decisions with dates: ../PLAN.md §2. Shared
schema: config.md; output: output.md.

## 1. What it is

`pun analyse` (alias `pun analyze`) answers one question: is this code wrong? It reports type errors, undefined symbols,
impossible conditions, dead code that needs types to spot, and misuse of Laravel. It never edits a file (commands.md).

It replaces:

| Upstream | What comes over |
| --- | --- |
| PHPStan core | every rule, natively (decided). The levels become presets (decided) |
| Larastan | built in, nothing to install (decided): models, relations, facades, container, config |
| phpstan-strict-rules, phpstan-deprecation-rules | the type-aware rules; strict-rules' syntax-only bans are lint rules |

The goal: on a project migrated from phpstan.neon, `pun analyse` reports the same problems as `phpstan analyse` (same
file, line and rule; messages may be reworded), so CI can switch.

## 2. How it works today

- **pun-phpstan** (100% coverage) ports PHPStan's own plumbing: the NEON parser, includes, parameters and schema
  validation, baseline read and write, `ignoreErrors`, `@phpstan-ignore` comments, PHPStan's type model and its
  `describe()` (so messages print types the way PHPStan does), and every built-in function and class exported from
  PHPStan's reflection into `data/symbols.json`.
- **pun-analyse** runs the rules:
  1. It parses with Mago's parser and resolves names.
  2. It collects every symbol of the project and vendor into Mago's codex.
  3. It checks each file in parallel with ported PHPStan rules.
  4. It drops what ignores and the baseline cover.
- **Level 0 is in progress.** It reports 54 PHPStan identifiers so far (undefined functions, classes, methods,
  properties and constants, `new`, typehints, `use`). It is checked against 142 of PHPStan's own rule test classes,
  with 37 known differing cases in `tests/fixtures/rules/gaps.txt`. On the corpus, the differences from PHPStan level 0
  are pinkary 1, larastan 6, pest 5 and laravel-framework 15, and Pun runs 5 to 25 times faster.
- **Not wired up yet.** There is no `pun analyse` command. The only entry point is
  `cargo run --example analyse <dir> <phpstan.neon>`, which reads phpstan.neon, used by `scripts/analyse-corpus.py`.
  The analyse agent stopped at 43d9857, with Larastan facade aliases uncommitted (left to analyse 6). The trait fix
  was redone in analyse 3a.
- **No type inference yet.** Level 0 mostly needs symbols, not types. Levels 2 and up need type inference (Mago's
  analyser, §7): the type of every expression, narrowing after `if`, `instanceof` and `is_*`, and generics from
  PHPDoc. Mago's analyser is a dependency but isn't used.

## 3. The config

The schema is config.md §3 and §5. pun-config reads it into `Config.analyse` (preset, paths, exclude, baseline, the
type switches and the enabled rules); `pun analyse` doesn't use it yet.

```toml
[analyse]
preset = "recommended"          # basic | essential | recommended | strict | max
exclude = ["tests/**"]          # added to [files] exclude; not analysed and not scanned for symbols
baseline = "pun-baseline.toml"

[analyse.rules]
"types/missing-iterable-value-type" = "off"      # on or off only; no warn
"undefined/method" = { magic = true }            # PHPStan's checkMagicMethods as a rule option
```

- **No levels.** `pun migrate` expands a PHPStan level into the exact rules it ran, each with its options. In
  interactive mode it offers the closest preset, with a count of the new problems the preset would report.
- **Laravel turns on by itself** when composer.lock has laravel/framework.
- PHPStan's level 7 to 10 behaviour is four switches: `union-types`, `nullables`, `explicit-mixed`, `implicit-mixed`
  (config.md §5). The presets turn them on.

## 4. Rules: what exists

There are 378 analyse rules in rules/mapping.toml: one per in-scope PHPStan `Rule` class (`[upstream.phpstan-classes]`,
from crates/pun-config/rules/upstream/phpstan-*.json): 326 from PHPStan core, 27 strict-rules, 7 deprecation-rules and
18 Larastan. A rule carries every identifier its class emits; each of the 688 identifiers of the family stays in the
mapping as a `variant` pointing at one class's rule, so migrated ignores and baselines keep their narrow scope. Of the
426 classes, 17 strict-rules and 1 core class are lint or format rules (rules.md O4) and 29 are unsupported (the
`phpstanApi.*` checks, playground-only and stub-only classes, one Larastan class that does not exist).

| | Count |
| --- | ---: |
| Analyse rules | 378 |
| By group: types / classes / dead-code / phpdoc / laravel / undefined / functions / operators / attributes / deprecated / other | 119 / 63 / 44 / 32 / 18 / 18 / 15 / 15 / 14 / 10 / 30 |
| By preset (from the class's PHPStan level): basic / essential / recommended / strict / max | 155 / 104 / 80 / 35 / 4 |
| PHPStan core identifiers by level: 0 / 1 / 2 / 3 / 4 / 5 / 6 / opt-in | 327 / 13 / 137 / 36 / 57 / 8 / 6 / 30 |
| Running today | 54 PHPStan identifiers (part of level 0) |

These groups still follow the old intent-style list. With topic groups (decided), `undefined/method` and
`types/argument` read naturally. The rename is config.md Build 1.

## 5. PHPStan's CLI, flag for flag

| PHPStan | Pun |
| --- | --- |
| `phpstan analyse [paths]` | `pun analyse [paths]` |
| `-c phpstan.neon` | `--config pun.toml`; phpstan.neon is read only by `pun migrate` |
| `-l 6` | `--preset strict` (no levels) |
| `--generate-baseline` | `pun analyse --generate-baseline` (writes pun-baseline.toml) |
| `--error-format=json/table/github/gitlab/checkstyle/junit/raw/teamcity` | `--format`, the same formats |
| `--memory-limit`, `--xdebug`, `--debug` | not needed |
| `clear-result-cache` | `--no-cache` / `pun cache clear` |
| `--pro`, `--watch` | `--watch` like `pun format` |

## 6. Build

Done: config, ignores and baselines read and written in PHPStan's format; the built-in symbols; the type model and
descriptions; 54 level-0 identifiers; parallel scanning. The steps, after config.md Build 1-3:

0. **Engine spike, alongside PLAN.md Step 1** (Pushpak, 2026-10-17): run mago-analyzer inside pun-analyse on Pinkary
   and compare it with PHPStan, and write one Larastan provider (model attributes from migrations) through Mago's
   plugin API without forking Mago.
   *Done when* it shows which remaining level-0 rules Mago already covers, so step 3 writes no rule twice.
   *Result* (2026-10-17, ../reports/analyse-spike.md): Mago covers 11 of the 116 remaining level-0 Pun rules, part
   of 32, none of 73. The Larastan provider works through `register_expression_hook` with no fork. Mago's codes fire
   far more widely than PHPStan's level-0 checks (421 issues on Pinkary, where PHPStan + Larastan report 0), so §7's
   "report a Mago code under the Pun rule" needs Pushpak's look: the report proposes Pun checks that read Mago's types,
   using a Mago issue directly only where it matches PHPStan.
1. **Wire it up.** The `pun analyse` command, `[analyse]` from pun.toml, the five presets selecting rules, the output
   (output.md: problem blocks, always errors), PHPStan's machine formats, agent output.
   *Done when* `pun analyse` on Pinkary prints the level-0 errors with PHPStan's exit codes.
2. **Pun's own baseline:** `pun analyse --generate-baseline` writes `pun-baseline.toml` (mago-reporting baseline,
   loose by default); a run reads it and reports fixed entries.
3. **Finish level 0** (after step 0). The remaining ~100 level-0 rules, an empty `tests/fixtures/rules/gaps.txt`, and corpus diffs at
   0 (pinkary 1, larastan 6, pest 5, laravel-framework 15 today). The trait-method work is redone (3a: traits in
   their class's context as NodeScopeResolver::processTraitUse() does, only for traits in analysed files); the
   facade-alias work belongs to Larastan (step 6).
4. **Type inference on Mago's analyser** (§7): expression types, narrowing, PHPDoc generics, mapped to PHPStan's type
   descriptions. Everything from `essential` up depends on it.
5. **Levels 1 to 10 as rules,** with the four switches, grouped into the presets basic, essential, recommended, strict,
   max. Then strict-rules and deprecation rules as plain rules.
   *Done when* each preset reports the same errors as its PHPStan level band on the corpus.
6. **Larastan built in** (§8): the Laravel knowledge (models, casts, relations, builder, collections, facades,
   container, helpers, macros), its 20 rules, then the seven improvements.
7. **Pest built in** (§9), with the PHPUnit typing it needs. **After 1.0** (Pushpak, 2026-10-18): until then test
   files are analysed as plain PHP; `[analyse] exclude` globs leave them out entirely.
8. **Result cache:** a warm run re-checks only changed files and the files that depend on them.
9. **`pun migrate` for phpstan.neon** (migrate.md Build 5). Coverage back to 100% throughout.

## 7. How PHPStan and Mago analyse, and what Pun builds on (2026-10-17)

**PHPStan.**
- `NodeScopeResolver` walks each file's syntax tree (nikic/php-parser) and carries a `Scope`, which holds the type of
  every variable at that point.
- `TypeSpecifier` narrows types after `if`, `instanceof`, `is_*` and assertions.
- Rules are small classes that get a node plus the scope and return errors with an identifier. Levels just choose
  which rules run and how strictly types are compared.
- Extensions (dynamic return types, property and method reflection) are how Larastan teaches it Laravel's magic.
- Baselines are a NEON list of (message, identifier, path, count).

**Mago's analyser** (mago-analyzer 1.57, about 106k lines of Rust, modelled on Psalm/Hakana):
- It walks the same kind of tree with a block context holding variable types.
- A reconciler with formula algebra does narrowing, as Psalm does. Generics and templates come from PHPDoc.
- It reports issues by code.
- A plugin system has providers (function, method and property return types, assertions, throws) and hooks (calls,
  expressions, statements, declarations). It already ships library plugins for the stdlib, PSL and PSR container.
- mago-reporting has a baseline: strict (exact lines) or loose (counts per file, code and message, the default), with
  pruning of outdated entries.

**Pun's choice (assistant's call, delegated by Pushpak).** Build analyse on Mago's analyser, not a port of PHPStan's
inference.
- Pushpak relaxed the two things that needed a PHPStan port: messages can be reworded, and migrate doesn't carry
  baselines over. What must hold is one Pun rule for every PHPStan rule, and that already holds in rules/mapping.toml
  (every one of the 559 identifiers maps to a Pun rule, or is listed as dropped or unsupported with a reason).
- Mago's inference, narrowing and generics exist today. A port of PHPStan's would be the largest single piece of
  work in Pun.
- **Rules** (Pushpak, 2026-10-18, replacing "report a matching Mago issue under the Pun rule"): every PHPStan rule is
  a Pun check that reads Mago's inferred types, the same way the 54 level-0 rules read Mago's codex. Mago's issue codes
  are broader than PHPStan's checks (forwarding them gave 226 false errors on Pinkary at level 0, reports/analyse-spike.md),
  so a Mago issue is used directly only where the spike's table says `yes` with no extras.
- **Laravel** (Larastan's knowledge) is a built-in Mago plugin in Pun. Providers give return and property types for
  models, relations, facades, the container and config; hooks add the Laravel-specific checks.
- **Parity test:** PHPStan's own rule fixtures and the corpus, compared on file, line and Pun rule, not message text.
  Differences go in gaps.txt, as today.
- **Baseline:** `pun analyse --generate-baseline` writes pun-baseline.toml on mago-reporting's baseline, loose by
  default. `pun migrate` does not copy phpstan-baseline.neon; it tells the user to generate a new one.
  `ignoreErrors` in phpstan.neon is config, not a baseline, so migrate still converts it to Pun rule IDs.
- **Kept from pun-phpstan:** the NEON reader, includes and `ignoreErrors` handling (for migrate), and
  `@phpstan-ignore` comment parsing (migrate rewrites those comments). Its PHPStan type model and `describe()` are no
  longer needed for messages.
- **Risk:** Mago infers differently from PHPStan in edge cases, so some PHPStan errors will be missing or extra. The
  parity test measures this, and each gap gets fixed in a Pun check or a Mago plugin.

## 8. Larastan, built in (Pushpak: "support Larastan, and improve on it")

Larastan has two halves, and without the first one every Laravel app drowns in false "undefined method" and
"undefined property" errors on its models. Pun builds both into the Laravel plugin (§7). It turns on by itself when
composer.lock has laravel/framework.

**1. Laravel knowledge: types PHPStan can't see.**

| Area | What Pun must know (Larastan's extensions) | Where it comes from |
| --- | --- | --- |
| Model attributes | every column as a typed property; `$casts` / `casts()` (enums, dates, custom casts); accessors and mutators (`get…Attribute` and `Attribute::make`); `$appends` | migrations (`Schema::create` / `table`), or the schema dump in `database/schema/*.sql`, plus the model class |
| Relations | relation properties (`$user->posts` is `Collection<Post>`), generic relation types (`HasMany<Post, User>`), `withCount` / `withSum` attributes | the relation methods' return types and bodies |
| Query builder | `Model::where(...)` forwarded to `Builder<Model>`, dynamic `whereEmail()`, local scopes (`scope…` and `#[Scope]`), custom builders (`newEloquentBuilder`, `#[UseEloquentBuilder]`) | the model and its builder class |
| Collections | `Collection<TKey, TValue>` generics through `map`, `filter`, `pluck` and higher-order proxies (`$users->each->notify()`); custom collections (`#[CollectedBy]`) | Laravel's own PHPDoc generics, plus the proxies |
| Facades | `Cache::get` resolved to the class behind the facade, including real-time facades | each facade's accessor, resolved through the container |
| Container | `app(Foo::class)`, `app('cache')`, `resolve()`, `App::make()` return the bound type | class names, and the framework's own bindings list |
| Helpers | `auth()->user()` is the configured user model; `config('app.name')` returns the type of the value in config/; `request()`, `view()`, `response()`, `now()` (Carbon or the configured date class) | config/auth.php, config/*.php, the framework |
| Macros | `Str::macro`, `Collection::macro` and `Builder::macro` calls become real methods | the `macro(...)` calls in service providers |
| Factories, requests, jobs | `User::factory()` is `UserFactory`; `FormRequest::validated()` and `safe()`; `dispatch()` arguments match the job's constructor | factories, form requests, jobs |

None of this needs PHP to run: Pun reads the files.

**2. Larastan's 20 rules.** Already mapped to Pun analyse rules (rules/mapping.toml): `undefined-relation`,
`undefined-model-append`, `undefined-console-argument` and `-option`, `unexpected-dispatch-arguments`,
`no-env-outside-config`, `redundant-collection-call`, `no-model-make`, `unused-view`, `missing-translation`,
`octane-compatibility` and the rest.

**3. Improvements over Larastan** (all seven ship in 1.0; things Pun can do because it reads the whole project quickly):
- **Typed config:** `config('services.stripe.key')` has the real type of that value, and a missing key is an error.
- **Routes:** `route('posts.show')`, `to_route()` and `redirect()->route()` with a name that doesn't exist is an
  error, and so are missing route parameters.
- **Views:** `view('emails.welcome')` with no such Blade file is an error. Larastan only reports views nobody uses.
- **Translations:** `__('auth.failed')` and `trans()` keys checked against lang/, for every locale.
- **Env:** `env('STRIPE_KEY')` keys checked against `.env.example`.
- **Inertia:** `Inertia::render('Posts/Show')` with no such page component is an error.
- **No stale model data:** Larastan needs ide-helper or a fresh schema dump to stay right. Pun re-reads migrations on
  every run, and the cache makes that free.

**Testing:** Larastan's own test suite (its `tests/Type` and `tests/Rules` fixtures) becomes the parity test, as
PHPStan's fixtures do for core. The corpus adds pinkary and larastan's own source.

## 9. Built-in support, not an extension system (2026-10-17)

**What Mago does.** Mago has no runtime extension system either. Its "plugins" are Rust modules compiled into the
binary (stdlib, PSL, PSR container, flow-php), each turned on by name in its config. Pun follows the same model: support
for a framework is Pun code inside the binary, turned on from composer.lock. A real extension system (third-party
plugins) is planned for later, not now (Pushpak).

**In the core for 1.0** (assistant's call within Pushpak's brief: "super Laravel support and Pest support", no
bloat):
- **Laravel**, all of §8.
- **Pest** (after 1.0, Pushpak 2026-10-18), as the official `pestphp/pest-plugin-phpstan` does: `$this` inside test closures is the test case,
  `expect()` chains are typed, and `uses()` / `pest()->extend()` classes are known. Pest runs on PHPUnit, so the small
  part of phpstan-phpunit that Pest needs comes with it: `createMock()` returning `MockObject&Foo`, and assertions
  narrowing types. This also covers plain PHPUnit tests.
- **Deprecation rules** (phpstan-deprecation-rules, 10 identifiers). These are rules, not type knowledge, and Mago's
  codex already records `@deprecated`.
- **strict-rules** stays as plain rules: 27 analyse identifiers and 16 that became lint rules. They add no type
  knowledge, so they add no bloat.

**Later, as extensions:** Mockery, Carbon macros, Livewire, Filament and the like. `pun migrate` names each one as not
supported yet. The same goes for a project's own PHP rules and type extensions. `stubFiles` keep working.

## Decisions

From earlier rounds (2026-10-17, Pushpak):
- Every PHPStan core rule runs natively; Larastan is built in.
- No levels. Named presets for new projects; migrate writes exact rules and offers the closest preset.
- Laravel, Pest and PHPUnit turn on automatically from composer.lock.
- Analyse never edits files; `pun check` runs it after lint and format.

Answered 2026-10-17 (Pushpak):
- **Engine: assistant's call.** The requirement is one-to-one rule compatibility, so that every PHPStan rule has a Pun
  rule in the mapping. The choice: Mago's analyser (§7).
- **Messages:** Pun may reword them.
- **Baseline:** its own feature (`pun analyse --generate-baseline`). Migrate never copies a PHPStan baseline.
- **Five presets, the top one called `max`** (Pushpak). Proposed bands, by the PHPStan levels each covers:
  `basic` (0-1), `essential` (2-3: PHPDoc and return types), `recommended` (4-5: dead code and argument types; the
  default), `strict` (6-8: missing types, union types and nulls), `max` (9-10: mixed, like PHPStan's `level: max`).
  Each includes the one below. The rules' `tier` field in mapping.toml moves to these five names with the rename.
- **Extensions:** no extension system now (later). Built in: Laravel (full, §8), Pest (with the PHPUnit part it
  needs), deprecation rules and strict-rules as plain rules (§9). Everything else is named as unsupported.
- **All seven Larastan improvements (§8.3) ship in 1.0.**
- **Analyse has no `warn`.** A problem analyse reports is an error; it never edits, so there is nothing softer to say.
  A rule is either on or off: `[analyse.rules] "types/missing-iterable-value-type" = "off"`, or `true` with options
  in a table. `off` stays because PHPStan users already turn rules off (by level or `ignoreErrors`), and migrate needs
  it to keep a project's current rule set.
- **Result cache before 1.0**, so a warm run re-checks only changed files and their dependents (assistant's
  default).
- **A project's own PHPStan PHP code:** named as unsupported by migrate; `stubFiles` keep working.
- **Larastan's model knowledge** comes from migrations and the schema dump, read without running PHP (§8).

Assistant's calls in the engine spike (Build 0, 2026-10-17; the plan was silent):
- The Larastan provider is one Mago `ExpressionHook` that retypes `$model->column` after Mago's `Model::__get()`.
  Mago 1.57 has no property-type provider for magic properties. A declared property or an `@property` tag wins, as
  PHPStan's own reflection comes before Larastan's extension.
- Accessors (`get…Attribute()`, `Attribute` methods; Larastan's ModelAccessorExtension) stay out of the spike; those
  attributes keep Mago's `mixed`, and the test names them.
- Where Mago's analyser panics on a debug assertion, `run_mago()` skips that file, as `rules::infer` already does.
- The coverage verdict counts a PHPStan error as found when a Mago issue of a code mapped to the same Pun rule is in
  the same file within 10 lines, or a code reviewed by hand as the same check (`SAME_CHECK` in scripts/analyse-spike.py).
- tests/fixtures/rules/gaps.txt was out of date at 915534e (3 listed cases pass, 5 unlisted fail since 138dab1's
  trait-context change). The spike updates the list so the suite is green, with the reason; fixing them is Build 3.
  Analyse 3a fixed the 5 (assistant's calls, where the port stops short of PHPStan: `@method` tags count only for
  `$this->` calls (CallMethodsRule), not static calls yet; `$this instanceof X` narrows `$this` to `never` only for
  those calls; a trait alias's new visibility (`foo as protected`) is not applied to the walked method).

Assistant's calls in analyse 3b (the first 13 level-0 declaration and keyword rules, 2026-10-18; the plan was silent):
- DeclareStrictTypesRule prints a constant value by its resolved name (`\FOO`) and any other value as written, in
  place of a port of php-parser's printer; the oracle agrees on ints, strings and constants.
- `Keywords/DeclareStrictTypesRuleTest::testNested` is in gaps.txt: PHPStan's own level-0 run aborts that file with
  an internal error (MissingReturnRule), so it reports none of its errors. Pun reports both declares, as the
  RuleTestCase expects.
- A `(void)` cast that starts an expression statement counts as first-level even when Mago parses it as the input of
  `|>` (`(void) 5 |> f(...);`), since PHP's grammar has `(void) expr;` as a statement.
- A trait method's alias in ConstructorReturnTypeRule comes from the class's alias map, not from the `use` block of
  that trait alone.
- BacktickRule also reports an empty pair of backticks, where PHPStan stops the file with an internal error.
- Verify (2026-10-18): PHPStan runs no rule on statements after a terminating one (`return; echo __CLASS__;` reports
  nothing), while Pun's walk checks dead code for every rule, these 13 included. That is an engine gap (unreachable
  statements, later with never-returning calls from inference), not a bug in this batch; it shows only on dead code.

Assistant's calls in analyse 3c (class-member declaration rules, 2026-10-18; the plan was silent):
- An enum case's value is the constant Mago's codex infers for it (class constants and `X::Case->value` followed
  through the codebase); float literals and `self::Case->value`, which the codex leaves untyped, are read from the
  case's own expression. A value neither gives (an array, a float constant) reports nothing.
- `PHP_VERSION_ID` narrowing for FinalPrivateMethodRule's `$scope->getPhpVersion()` is a scope guard: `<`, `<=`, `>`,
  `>=` against an integer, intersected across nested conditions; an `||` keeps the hull of both ranges.
- `property.readOnlyNotSupported` is not reported: below PHP 8.1 php-parser lexes `readonly` as a name, so PHPStan
  reports a parse error and never reaches ReadOnlyPropertyRule's check.
- `isImmutable()` reads a parent class's PHPDoc from its file (an analysed one's source, otherwise from disk).
- php-parser's modifier errors (`abstract final` property) are not emulated: Mago parses them, so PropertyInClassRule
  reports where PHPStan reports only `phpstan.parse` (2 fixture cases in gaps.txt).

Assistant's calls in analyse 3d (syntax-only level-0 rules, 2026-10-18; the plan was silent):
- `[...]` inside `list()` is an array literal, not a nested destructuring, as php-parser parses it (oracle: `list($a[], [$a[]])`
  reports `assign.invalidExpr` and `offsetAccess.noDim`).
- `break N` with an N that does not fit a PHP int counts as level 1, because php-parser parses it as a float (oracle-confirmed).
- InvalidTypesInUnionRule prints built-in type names lowercased, as php-parser lowercases them (oracle: `int|VOID` gives `Type void`).
- A promoted constructor property with an invalid type is reported twice (parameter and ClassPropertyNode), as PHPStan does.
- To find by-reference callees, TypedExpressions also asks Mago for the type of method-call objects and static-call class
  expressions, only when an argument is a `[]` fetch or a dim fetch on `$this`.

Assistant's calls in analyse 3e (the attribute rules on AttributesCheck, and FunctionDefinitionCheck's `#[NoDiscard]`
check, 2026-10-18; the plan was silent):
- `#[Attribute]`'s flags are the type Mago's scan gives the constructor's first argument after ArgumentsNormalizer
  (first positional, else `flags:`, else the first unknown named one), with class and global constants expanded
  through the codebase; anything but one integer makes the class no attribute class, as PHPStan's
  ConstantIntegerType check does. A built-in class Mago's prelude lacks counts as no attribute class.
- The attribute name check (class.nameCase, class.prefixed) waits for ClassNameCheck's port: other level-0 rules emit
  those identifiers too. attribute.deprecated waits for deprecation-rules (`deprecationRulesInstalled` is false).
- constant.attributesRuleCannotRun is never reported: Pun behaves as PHPStan on a PHP 8.5 runtime.
- `#[NoDiscard]` on a function or method reads the native return type, then (with none) the PHPDoc type Mago read;
  constructors, `__destruct()`, `__unset()`, `__wakeup()`, `__clone()`, `__set()` and `__unserialize()` are void, as
  in PhpMethodFromParserNodeReflection.

Assistant's calls in analyse 3f (class-hierarchy declaration rules, hierarchy.rs, 2026-10-18; the plan was silent):
- `isFinalByKeyword()` of a user class reads the `final` modifier from the declaration's source (after its attributes,
  outside comments): Mago's FINAL flag also holds `@final`, even a comment between the attributes and the keyword,
  which PHPStan does not read as the class's PHPDoc (oracle-confirmed). Without the source (PHPStan's API stub) the
  flag is the keyword.
- A method is abstract when declared `abstract` or in an interface (BetterReflection); Mago also flags a body-less
  method, so the modifier is read from the source.
- MissingMethodImplementationRule resolves each method name with `members::method()` (class, traits, parents, then
  interfaces) instead of Mago's declaring ids, which also hold `@method` tags; a trait's abstract method is satisfied
  by a parent's concrete one, and an interface's by a concrete method of any trait the class or a parent uses
  (Mago's declaring ids let another trait's `@method` tag hide it: Carbon's `tomorrow()` on laravel-framework), as
  in PHP. A class whose parent or interface is of the wrong kind (a class implementing an enum) is skipped: PHP
  refuses that hierarchy and PHPStan reports no missing method. `isProvidedByBuiltinAncestor()` is not ported:
  Mago's stubs give the built-in parents concrete methods, so no input reaches it.
- `instanceof.trait` describes only `$this` (`$this(Class)`) and `mixed`; other expression types wait for analyse 4.
- The ClassNameCheck part of the three ExistingClassIn* rules (`*.nameCase`, `class.prefixed`) is not ported (analyse 3i ported it).

Assistant's calls in analyse 3i (ClassNameCheck once for every level-0 caller, class_names.rs, 2026-10-19; the plan
was silent):
- `checkClassCaseSensitivity` is a bool on `rules::Checks`, copied from a new `Project::check_class_case_sensitivity`
  that is always false (level 0) until the presets set it; it is not read from the PHPStan config, and pun-config has
  no option. The callers that pass it upstream are FunctionDefinitionCheck (functions, methods, closures, arrow
  functions, hooks, `@phpstan-self-out`), ExistingClassInInstanceOfRule, CaughtExceptionExistenceRule and
  ExistingClassesInPropertiesRule; the rest call with `true`. tests/class_names.rs proves the flag against a
  level-2 oracle run.
- `checkInternalClassCaseSensitivity` stays off, but PHPStan reflects four core attribute classes from its own
  stubs (stubs/runtime/Attribute84.php: Attribute, AllowDynamicProperties, ReturnTypeWillChange,
  SensitiveParameter), so they are not built-in to it and their case is checked (`#[\ATTRIBUTE]` reports at level 0).
  A probe of every class in data/symbols.json found no others.
- UseAliasVisitor is a per-namespace map of explicit `use X as Y` aliases; a pair whose node is the written name
  skips the case check when the name is one part written exactly as such an alias. Pairs on other nodes (an
  attribute, a parameter, a property, a `use` item) never skip, as their php-parser node carries no such attribute.
- StaticMethodCallCheck's case and prefix errors are dropped where PHPStan returns none: a missing method on a class
  with `__callStatic()` (reportMagicMethods is off at level 0), unless a guard or an `@method static` tag knows it.
- AccessStaticPropertiesCheck and StaticMethodCallableRule have no call site in Pun yet (their rules are not
  ported), so their ClassNameCheck calls wait for those rules.

Assistant's calls in analyse 3h (class-constant, PHP-version and literal-array declaration rules, constants.rs,
arrays.rs, 2026-10-19; the plan was silent):
- OverridingConstantRule is ported at level 0: its native and PHPDoc type checks (`classConstant.nativeType`,
  `classConstant.missingNativeType`, `classConstant.type`) run only with checkPhpDocMethodSignatures (level 3), so
  those identifiers come from ConflictingTraitConstantsRule at level 0. `classConstant.value` is also emitted by
  ValueAssignedToClassConstantRule (level 2); both are kept in IDENTIFIERS, as level 0 has no other source of them.
- An interface's constants are final before PHP 8.1 (BetterReflection's `isFinal()`, oracle-confirmed); otherwise a
  constant is final by keyword or `@final` (Mago's FINAL flag). The trait constants ConflictingTraitConstantsRule reads
  use the same flag for `isFinal()`.
- A trait constant's declaring class is the trait iterated (PHP's reflection copies a used trait's constants into the
  using trait); a parent's constant taken from a trait is declared by the class using the trait (oracle-confirmed).
- Native constant types are compared as PHPStan describes them, from the hint's source text through pun-phpstan's
  type resolver (class names stay as written, `self` resolves to the class).
- Constant values are compared only when the codex reduces both to a constant scalar; of duplicate declarations (a
  PHP error) the codex keeps the first and PHP's reflection the last, so DuplicateTraitDeclarationRuleTest::testBug14250
  stays in gaps.txt.
- ReadOnlyClassRule is ported with this batch because it also emits `classConstant.nativeTypeNotSupported` (the
  mapping.toml entry is right); below PHP 8.1 php-parser lexes `readonly` as a name, so only 8.1 reaches its check.
- DuplicateKeysInLiteralArraysRule knows a key's value for literals, `true`/`false`/`null`, user global and class
  constants (`self::`, `parent::`, named classes) and `X::class`; variables, calls, `static::X` and built-in constants
  are unknown keys. A key is printed as written, with class names and qualified constant names fully qualified, as
  php-parser's printer does after NameResolver (oracle-confirmed). DuplicateKeysInLiteralArraysRuleTest::testDuplicateKeys
  stays in gaps.txt for its variable keys.
- `new X(...)` is a Mago parse error; the recovered argument list (one unpacked error argument) is
  InstantiationCallableRule's node, and the `new` is then not analysed, as in NodeScopeResolver.
- PropertyAssignRefRule reads the holder's class from `$this` or Mago's type of a single named object; a dynamic
  property name and a closure bound with `Closure::bind()` are not ported. A property's set visibility is asymmetric
  only when narrower than its read visibility (`protected readonly` is `protected`, oracle-confirmed).

Assistant's calls in analyse 3j (anonymous-class names, anonymous.rs and relative_path.rs, 2026-10-19; the plan was
silent):
- An anonymous class's name is BetterReflectionProvider's `<parent>@anonymous/<path>:<line>`: the parent class as
  written (resolved), else the first interface, else `class`; `:<index>` when several start on one line
  (AnonymousClassVisitor, in source order). The line is php-parser's `Class_` start: the attributes, else the
  modifiers, else the `class` keyword.
- The path is FuzzyRelativePathHelper's: the analysed paths' common directory below the working directory (the process
  cwd, as PHPStan's `currentWorkingDirectory`), else ParentDirectoryRelativePathHelper's path from the cwd. Windows
  drive prefixes are not ported.
- A trait walked in an anonymous class's context is labelled `T.php (in context of class@anonymous/...)`, without
  `class ` (MutatingScope::getFileDescription()).

Assistant's calls in analyse 3k (property access, property_access.rs, 2026-10-19; the plan was silent):
- AccessPropertiesRule and AccessPropertiesInAssignRule check `$this->x` only (checkThisOnly below level 2). A class
  with a native `__get`/`__set` is never reported (reportMagicProperties is off at level 0). The narrowing that hides
  a read is ported as scope keys: assignments, isset/empty chains, comparison and instanceof operands, a function
  call's first argument, a true `property_exists($this, 'x')`, `if`, `while` bodies, `?:` and `&&`.
- AccessStaticPropertiesRule is ported for `self::`, `static::`, `parent::` and named classes. `$expr::$x` and a
  dynamic name wait for expression types. `isset(self::$x)` on a class without `$x` is still reported, and so is a
  read after `self::$x = ...` (both oracle-checked). A static hooked property counts as static, as in PHPStan's
  reflection, although Mago drops the flag.
- `$this++` or `--$this` stops the `$this` checks for the rest of the function. `$this[] = ...` on a class without
  ArrayAccess, which PHPStan then describes as `ArrayAccess&C`, is not ported.
- `(function () {...})->bindTo()` keeps the scope's `$this` (oracle-checked). Only `->call()` and `Closure::bind()`
  rebind it. An anonymous class's constructor arguments are walked in the enclosing scope.
