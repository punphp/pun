#!/usr/bin/env bash
# Reproduce the pinned formatting oracle in .oracle/ from scratch (versions: context/reference/oracle.md).
#   .oracle/pint          laravel/pint at the ref Pinkary's composer.lock pins (dev-feat/blade-support),
#                         with dev deps installed (bundled PHP-CS-Fixer lives in its vendor/).
#                         Oracle binary: php .oracle/pint/builds/pint (the committed PHAR Pinkary runs).
#   .oracle/php-cs-fixer  PHP-CS-Fixer source + tests at the exact version Pint bundles,
#                         with dev deps (PHPUnit) for fixture export. Binary: php .oracle/php-cs-fixer/php-cs-fixer
#   .oracle/rector        rector-src at the rector/rector tag 2.8.4 (the latest release), for the rules'
#                         fixtures (rules-tests/). No composer install: the runnable Rector is .oracle/rector-bin.
#   .oracle/rector-bin    rector/rector 2.8.4 (the released, scoped build), pinned by scripts/oracle-rector/composer.lock.
#                         Binary: php .oracle/rector-bin/vendor/bin/rector
#   .oracle/rector-ecosystem  rector/rector 2.8.4 + driftingly/rector-laravel 2.7.3 + pestphp/pest-plugin-rector
#                         5.x-dev@8c1b780, pinned by scripts/oracle-rector-ecosystem/composer.lock (the rule mapping's
#                         rector-laravel and Pest inventories; rector-phpunit is bundled in rector/rector).
#   .oracle/phpstan-src   phpstan-src at the tag 2.3.1 (Rule classes and tests; the PHPStan inventories).
#   .oracle/phpstan       phpstan/phpstan 2.3.1 + larastan + strict/deprecation rules, pinned by scripts/oracle-phpstan/composer.lock.
#   .oracle/phpstan-phar  that PHAR extracted, for reading (export-phpstan-api-stub.php, export-phpdoc-parser-fixtures.php).
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
PINT_REF=cef1b260c611f92e66902d678092e26932c9ea00
FIXER_REF=0ee88422118f3cc59c8c3def222ba7f1493b6d5b # v3.95.17
RECTOR_REF=0ddd8e3c8de9c32e9c12396a7f50234bdba559d0 # rector-src 2.8.4
PHPSTAN_SRC_REF=cb3b7e99489b2e3789ee2eec10501cb7a72580ea # phpstan-src 2.3.1

fetch() { # dir url ref
  if [ "$(git -C "$1" rev-parse HEAD 2>/dev/null)" = "$3" ]; then return; fi
  rm -rf "$1" && git init -q "$1"
  git -C "$1" fetch -q --depth 1 "$2" "$3"
  git -C "$1" -c advice.detachedHead=false checkout -q FETCH_HEAD
}

mkdir -p "$root/.oracle"
fetch "$root/.oracle/pint" https://github.com/laravel/pint.git "$PINT_REF"
fetch "$root/.oracle/php-cs-fixer" https://github.com/PHP-CS-Fixer/PHP-CS-Fixer.git "$FIXER_REF"
fetch "$root/.oracle/rector" https://github.com/rectorphp/rector-src.git "$RECTOR_REF"
fetch "$root/.oracle/phpstan-src" https://github.com/phpstan/phpstan-src.git "$PHPSTAN_SRC_REF"
(cd "$root/.oracle/pint" && composer install -q --no-interaction --no-progress)
(cd "$root/.oracle/php-cs-fixer" && composer install -q --no-interaction --no-progress)

# PHPStan oracle (pun-phpstan): pinned by the committed lock file.
mkdir -p "$root/.oracle/phpstan"
cp "$root/scripts/oracle-phpstan/composer.json" "$root/scripts/oracle-phpstan/composer.lock" "$root/.oracle/phpstan/"
(cd "$root/.oracle/phpstan" && composer install -q --no-interaction --no-progress)
rm -rf "$root/.oracle/phpstan-phar"
php -r '(new Phar($argv[1]))->extractTo($argv[2]);' "$root/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar" "$root/.oracle/phpstan-phar"

# Rector oracle (pun-lint): the released rector/rector, pinned by the committed lock file.
mkdir -p "$root/.oracle/rector-bin"
cp "$root/scripts/oracle-rector/composer.json" "$root/scripts/oracle-rector/composer.lock" "$root/.oracle/rector-bin/"
(cd "$root/.oracle/rector-bin" && composer install -q --no-interaction --no-progress)

# Rector ecosystem (the rule mapping's inventories, crates/pun-config/rules/upstream/): pinned by the committed lock file.
mkdir -p "$root/.oracle/rector-ecosystem"
cp "$root/scripts/oracle-rector-ecosystem/composer.json" "$root/scripts/oracle-rector-ecosystem/composer.lock" "$root/.oracle/rector-ecosystem/"
(cd "$root/.oracle/rector-ecosystem" && composer install -q --no-interaction --no-progress)

php "$root/.oracle/pint/builds/pint" --version
php "$root/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar" --version
php "$root/.oracle/php-cs-fixer/php-cs-fixer" --version
php "$root/.oracle/rector-bin/vendor/bin/rector" --version
bundled=$(php -r 'Phar::loadPhar($argv[1], "p.phar"); foreach (json_decode(file_get_contents("phar://p.phar/vendor/composer/installed.json"), true)["packages"] as $p) if ($p["name"] === "friendsofphp/php-cs-fixer") echo $p["source"]["reference"];' "$root/.oracle/pint/builds/pint")
[ "$bundled" = "$FIXER_REF" ] || { echo "PHAR bundles php-cs-fixer $bundled, expected $FIXER_REF" >&2; exit 1; }
echo "oracle ok"
