# Plan: the package manager (`pun install`, `pun update`, `pun require`, `pun remove`)

Status: **planned, not built** (2026-11-14). Decisions with dates: §6. Beyond Composer: §7. Open questions: §8.

## 1. What it is

Pun installs a project's dependencies the way Composer does, much faster. It reads `composer.json`, reads and writes
`composer.lock`, and fills `vendor/` with the same files `composer install` writes, so a team can mix `pun install` and
`composer install` on one project and CI can switch without touching the repository. The proof is a parity harness:
on every corpus project, `pun install` and `composer install` give byte-identical `vendor/` trees, and `pun update`
writes the same `composer.lock` as `composer update`.

The everyday commands are top-level, with Composer's names and flags:

| Command | Composer | What it does |
| --- | --- | --- |
| `pun install` | `composer install` | Installs `composer.lock` (resolves and writes it when there is none) |
| `pun update [packages]` | `composer update` | Resolves again, writes `composer.lock`, installs |
| `pun require <pkg[:constraint]>` | `composer require` | Adds to `composer.json`, then a partial update |
| `pun remove <pkg>` | `composer remove` | Removes from `composer.json`, then a partial update |
| `pun composer <command>` | `composer <command>` | Every other Composer command: `dump-autoload`, `show`, `outdated`, `why`, `why-not`, `audit`, `validate`, `licenses`, `run-script`, `check-platform-reqs`, `config`, ... |

`pun composer install` and `pun install` are the same command. A `pun composer` command Pun has not built yet is a
run error (exit 2) that names it; Pun does not shell out to the Composer CLI.

## 2. What we learnt from phast and Composer

- **phast** (github.com/phast-sh/phast, Rust, MIT or Apache-2.0, 9 crates, about 45k lines) is the speed reference:
  parallel downloads and extraction, a PubGrub resolver, a TOML manifest and lock of its own (`phast migrate` converts
  `composer.json` once and never reads it again), its own autoloader under `vendor/phast/` with a
  `\Composer\InstalledVersions` shim, a scripts runtime with a `\Composer\Script\Event` shim, and no Composer plugins.
  Pun keeps Composer's files instead, so phast's formats, resolver and autoloader are not what Pun ships. phast is a
  benchmark and a design reference only: no phast code is copied into Pun (§6).
- **Composer** (2.x, MIT, about 79k lines of PHP) is the oracle. The parts Pun ports: `DependencyResolver` (8k lines:
  Pool, PoolBuilder, PoolOptimizer, RuleSetGenerator, Solver, DefaultPolicy, Transaction), `Autoload` (2k),
  `Installer` and `Downloader` (7k), the parts of `Repository` and `Package` that read Packagist metadata and write the
  lock, and `composer/semver`. Its 231 installer fixtures (`tests/Composer/Test/Fixtures/installer/*.test`: a
  composer.json, an optional lock and installed repo, the expected operations and lock) and its autoload fixtures are
  the test source, exported to JSON as the fixer fixtures are.
- **Real projects need plugins and scripts.** Every Laravel corpus project allows `pestphp/pest-plugin` and
  `php-http/discovery`; dev-null.sh adds `laravel/vet`; larastan allows `dealerdirect/phpcodesniffer-composer-installer`.
  `post-autoload-dump` runs `Illuminate\Foundation\ComposerScripts::postAutoloadDump` (a PHP callback that takes a
  Composer `Event`) and `@php artisan package:discover`, which reads `vendor/composer/installed.json`. Without them a
  Laravel app is broken after install.

## 3. The decided behaviour

- **Composer's files are the source of truth.** `composer.json` is read and, for `require` and `remove`, edited in
  place keeping its formatting (Composer's JsonManipulator). `composer.lock` is written byte for byte as Composer
  writes it, `content-hash` included. There is no `pun.toml` section for dependencies and no Pun lock file.
- **`vendor/` is Composer's layout:** package directories, `vendor/composer/` (`installed.json`, `installed.php`,
  `InstalledVersions.php`, `ClassLoader.php`, `autoload_*.php`, `platform_check.php`, `LICENSE`), `vendor/autoload.php`
  and the `vendor/bin` proxies, all byte-identical to Composer 2's. `ClassLoader.php` and `InstalledVersions.php` are
  Composer's own files, embedded with its MIT notice.
- **The same resolution as Composer.** The solver is a function-by-function port of Composer's SAT solver and policy,
  so `pun update` picks the versions Composer picks, including on conflicts. PubGrub explains conflicts better; Pun's
  conflict message is its own human output over the same problem set (output.md).
- **Composer's environment:** `COMPOSER_HOME`, `COMPOSER_CACHE_DIR` (the same cache, so Pun and Composer share
  downloads and metadata), `auth.json` (project and home) and `COMPOSER_AUTH`, `config.platform`, `minimum-stability`,
  `prefer-stable`, `preferred-install`, `vendor-dir`, `bin-dir`, `optimize-autoloader`, `classmap-authoritative`,
  `apcu-autoloader`, `sort-packages`, `allow-plugins`, `process-timeout`, `audit` and `policy` settings.
- **Repositories:** Packagist (the v2 metadata API), any Composer repository (Private Packagist, Satis), `path`,
  `package`, `vcs`/`git`/`github`/`gitlab` (through the host's API and git), and `artifact`. Not `svn`, `hg` or `fossil`.
- **The platform is the system PHP's.** Pun asks the `php` on the PATH once for its version, extensions and their
  versions, as Composer sees them, unless `config.platform` says otherwise. Scripts run on that PHP too.
- **Scripts and events run as Composer runs them:** shell commands, `@php`, `@composer` (which runs Pun's own
  command), `@putenv`, script references, and PHP callbacks. A callback gets a `Composer\Script\Event` from a small
  bundled PHP shim (Event, Composer, Config, IOInterface, the package objects a callback reads), enough for Laravel's
  `ComposerScripts`, Pest's and the common callbacks; the shim's API is tested against the corpus.
- **Composer plugins work.** An allowed plugin runs as it runs under Composer, through a plugin host:
  - The host is a PHP process on the system PHP running Composer's own classes (a pinned `composer.phar`, MIT,
    downloaded once with a checksum into Composer's cache, not bundled into the binary). It builds a real `Composer`
    object from the files Pun has written (`composer.json`, `composer.lock`, `vendor/composer/installed.json`),
    activates the allowed plugins through Composer's PluginManager, and dispatches the events Pun sends it.
  - Pun and the host talk over a pipe in JSON. Pun fires every event Composer fires, in Composer's order: the script
    events, the package events (`pre-package-install` and the rest), the installer events (`pre-operations-exec`), and
    the plugin events (`pre-pool-create`, `pre-file-download`, `post-file-download`, `command`, `init`). Where a
    plugin changes Pun's work (a pool filter like laravel/vet's, a custom installer's install path like
    composer/installers', a gated operation, a rewritten download URL), the host sends the change back and Pun applies
    it.
  - A plugin's commands (`Capable` with a CommandProvider, like Pest's `pest:dump-plugins`) run through the host as
    `pun composer <command>`.
  - The host starts only when an allowed plugin subscribes to an event the command fires, and once per command.
  - The plugins every Laravel project has get native ports that write the same files without PHP, so the common
    install starts no host: `pestphp/pest-plugin`, `php-http/discovery`, `phpstan/extension-installer`,
    `dealerdirect/phpcodesniffer-composer-installer`. A port is used only when its parity test against the plugin
    passes for that plugin version; otherwise the host runs the plugin.
  - A plugin `allow-plugins` denies, or that it does not list, is handled as Composer handles it (skipped, or the
    interactive prompt, or the non-interactive error).
- **Output:** machine formats (`--format=json` of `show`, `outdated`, `audit`, `licenses`), exit codes and the files
  Pun writes match Composer. A run error exits 2 where Composer exits 1 (output.md). The human output is Pun's own,
  with phases and timings (`Resolved 211 packages in 0.14s`), and agent mode is compact as for the other commands.
- **Speed is the point:** parallel HTTP/2 metadata and dist downloads, parallel extraction, Composer's shared cache,
  a no-op `pun install` that only checks the lock against `installed.json`, and classmap scanning with Pun's own lexer.
  `scripts/bench.py` gains `BENCH=install` (cold cache, warm cache, no-op) against Composer and phast.

## 4. Crates

- `pun-semver`: composer/semver (VersionParser, Constraint, MultiConstraint, Comparator, Semver, Intervals), with
  composer/semver's tests exported to JSON.
- `pun-composer`: everything without I/O to the network: composer.json and composer.lock models, the lock writer and
  `content-hash`, Packagist metadata (v2, minified), the solver (Pool to Transaction), the installed repository, the
  autoload dump, bin proxies, scripts and events, the native plugin ports. Network access goes through a trait,
  so the solver and installer tests run on recorded metadata.
- `pun-fetch`: HTTP (HTTP/2, rustls, gzip), Composer's cache directory, auth, dist downloads with checksums, archive
  extraction (Composer's top-level directory rule), git and VCS host APIs. HTTP is new to Pun: it adds reqwest and an
  async runtime to the binary, about 3 MB.
- `pun-composer`'s plugin host: the PHP bootstrap (Pun's own code, run with Composer's phar), the JSON protocol, and
  the event dispatch on the Rust side.
- `pun-cli`: the commands and their output.

## 5. Build

Each step lands with its tests green and its parity check, in this order. Install from a lock comes first: it is
what CI and most teammates run, and it needs no resolver.

1. **Oracle and corpus.** `scripts/oracle.sh` installs the pinned Composer 2.11.2 phar into `.oracle/composer`.
   `scripts/install-parity.sh <project>` runs `composer install` and `pun install` on scratch copies with an empty
   vendor and compares the trees (paths, contents, modes, symlinks). Composer's installer and autoload fixtures and
   composer/semver's tests are exported to JSON.
2. **`pun-semver`.** Done when composer/semver's tests pass.
3. **composer.json and composer.lock.** Models, `content-hash`, the lock writer, and `pun composer validate`. Done
   when every corpus lock round-trips byte for byte and `validate` matches Composer's messages and exit codes.
4. **`pun install` from a lock.** Download, cache, extract, `installed.json` and `installed.php`, bin proxies, path
   repositories (symlink or copy), source installs for `preferred-install: source` and dev branches. Done when
   install-parity gives identical vendor trees on pinkary, laravel-framework, larastan, laravel-pint and pest, except
   `vendor/composer/autoload_*` until step 5.
5. **`pun composer dump-autoload`.** PSR-4, PSR-0, classmap (with `exclude-from-classmap`), files, `-o`,
   `--classmap-authoritative`, `--apcu`, `platform_check.php`. Done when Composer's autoload fixtures pass and the
   corpus `vendor/composer/` is identical in all modes.
6. **Scripts and events.** The Event shim, every event `install` and `dump-autoload` fire, `pun composer run-script`.
   Done when pinkary's `post-autoload-dump` runs and `php artisan package:discover` writes the same
   `bootstrap/cache/packages.php` after `pun install` as after `composer install`.
7. **Plugins.** First the host with the script, package and installer events and plugin commands; then the native
   ports, each tested against the real plugin. Done when every corpus project and a plugin fixture set
   (pest-plugin, php-http/discovery, extension-installer, composer/installers, phpcodesniffer-composer-installer,
   laravel/vet, symfony/flex, a custom installer, a plugin command) give the same files as Composer.
8. **Metadata and the solver.** Packagist v2 and Composer repositories with the shared cache and conditional
   requests; Pool, PoolBuilder, PoolOptimizer, RuleSetGenerator, Solver, DefaultPolicy, the problem reports. Done when
   Composer's installer fixtures pass. The `pre-pool-create`, `pre-file-download` and `post-file-download` events
   reach the plugin host here.
9. **`pun update`, `pun require`, `pun remove`.** Full and partial updates (`--with-dependencies`,
   `--with-all-dependencies`, `--prefer-lowest`, `--minimal-changes`), `require` and `remove` with JsonManipulator.
   Done when `pun update` writes the same composer.lock as Composer on every corpus project from its composer.json,
   and on 20 recorded partial updates.
10. **The long tail of `pun composer`:** `show`, `outdated`, `why`, `why-not`, `audit`, `licenses`, `check-platform-reqs`,
    `config`, `init`, `create-project`, `global`, `status`, `reinstall`, `bump`, in that order, each with its machine
    format matched.
11. **Docs and bench.** A docs section for the package manager (commands, compatibility, plugins, what is not
    supported), `BENCH=install` results in `context/reports/scoreboard.md`.

## 6. Decisions

- Top-level verbs `install`, `update`, `require`, `remove`; `pun composer <command>` for the rest (Pushpak,
  2026-11-13).
- `composer.json` and `composer.lock` stay the source of truth; no Pun manifest or lock (Pushpak, 2026-11-13).
- Port Composer's solver, not PubGrub, so the lock matches Composer's (assistant's call, AGENTS.md "Match the upstream
  tool").
- Composer plugins must work (Pushpak, 2026-11-13): every allowed plugin runs through the plugin host on Composer's
  own classes; native ports only speed up the common ones.
- No phast code is copied (Pushpak, 2026-11-13). Pun's code is written from Composer's behaviour and its own design;
  phast is a benchmark and a reference for ideas only. Composer's own files that `vendor/` must contain
  (`ClassLoader.php`, `InstalledVersions.php`) and the phar the host runs keep Composer's MIT notice.
- `@composer` in a script runs Pun's command, not a Composer binary (assistant's call).
- Composer 2.11.2 is the pinned oracle, and the plugin host runs that same phar; Pun follows Composer's minor releases
  as it follows PHPStan's (Pushpak, 2026-11-14).
- A fresh clone needs no Composer: the standalone installer (`install.sh` on every release, foundations.md §4) gives a
  global `pun` that runs `pun install` (Pushpak, 2026-11-14). `pun install` in a project that requires `punphp/pun`
  rewrites the `vendor/punphp/pun` shim but not the binary it runs, which the shim keeps beside it; a test covers it.

## 7. Beyond Composer: what pnpm and Cargo add

Every feature here keeps the project working under plain Composer: it reads only Composer's files plus keys under
`extra.pun` (Composer ignores `extra`), and an unused feature changes nothing Pun writes. Each one is built after the
step it depends on.

1. **Workspaces (monorepos), as Cargo and pnpm have them.** After step 9.
   - The root `composer.json` opts in with `extra.pun.workspace.members`, a list of globs (`["packages/*", "apps/*"]`).
     Each member is a directory with its own `composer.json`. Pun checks that the root also lists the globs as `path`
     repositories, so `composer install` at the root resolves the same members. `pun workspace init` writes both.
   - One `composer.lock` and one `vendor/` at the root, as Cargo keeps one `Cargo.lock` and one `target/`. Members
     are installed as path packages (symlinked) and every member's `require` joins one resolution, so the whole
     repository gets one version of each package.
   - Every command finds the workspace root by walking up from the current directory, so `pun install` works in any
     member.
   - `-p <member>` (Cargo's flag; pnpm calls it `--filter`) picks members by name or glob: `pun require -p acme/billing
     guzzlehttp/guzzle` edits that member's `composer.json` and updates the root lock. Without a constraint, `require`
     uses the constraint other members already have for that package.
   - `pun composer run-script --workspace <script>` runs a script in every member that defines it, in dependency
     order, with independent members in parallel. `pun check`, `format`, `lint` and `analyse` take `--workspace` too
     and use each member's own `pun.toml`.
   - `pun composer validate --workspace` reports what Cargo's `workspace.dependencies` would have prevented: two
     members requiring different constraints for one package, and a member requiring another member at a version it
     does not have.
2. **A shared file store, as pnpm has.** After step 4. Every extracted file lives once in a content-addressed store
   in Pun's cache, and `vendor/` gets copy-on-write clones of it (APFS, btrfs, XFS), so the second project that needs
   `laravel/framework` 13.2.0 installs it without copying. The bytes and modes are Composer's, so install parity
   holds. Where the filesystem cannot clone, files are copied; hardlinks (pnpm's default) are opt-in through
   `extra.pun.link = "hardlink"`, because editing a hardlinked file in `vendor/` edits every project's copy.
3. **`--locked` and `--offline`, as Cargo has them.** After step 4. `pun install --locked` fails when there is no
   `composer.lock` or its `content-hash` does not match `composer.json` (Composer only warns). It is the default when
   `CI` is set, as pnpm's `--frozen-lockfile` is. `--offline` resolves and installs from the cache only and names the
   first package it cannot find there.
4. **`pun x`, as `pnpm dlx` and `cargo install` have it.** After step 9. `pun x laravel/installer new app` resolves
   the package into a cached scratch project and runs its binary without touching the current project.
   `pun x pkg@1.2` pins a version.
5. **A minimum release age, as pnpm has it.** After step 8. `extra.pun.minimum-release-age` (minutes) makes the
   resolver ignore versions published more recently, against compromised releases that are pulled within hours.
   Off by default, because with it `pun update` can pick older versions than `composer update`.
6. **Patches, as `pnpm patch` and Cargo's `[patch]` have them.** After step 7. `pun patch <package>` copies the
   package to a scratch directory to edit, and `pun patch --commit <dir>` writes a unified diff to `patches/` and
   registers it under `extra.patches` in the format `cweagans/composer-patches` reads. The same patch then applies
   under Composer with that plugin, and Pun applies it natively.

## 8. Open questions (needs Pushpak)

1. **Workspace members.** `extra.pun.workspace.members` plus matching `path` repositories (§7.1, closest to how
   Composer monorepos work today), or only the `path` repositories, with every path package treated as a member?
2. **Which §7 features land before 1.0.** Proposed: `--locked` and `--offline` with step 4, workspaces right after
   step 9, and the rest after 1.0.
