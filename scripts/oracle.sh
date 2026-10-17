#!/usr/bin/env bash
# Reproduce the pinned formatting oracle in .oracle/ from scratch (versions: context/reference/oracle.md).
#   .oracle/pint          laravel/pint at the ref Pinkary's composer.lock pins (dev-feat/blade-support),
#                         with dev deps installed (bundled PHP-CS-Fixer lives in its vendor/).
#                         Oracle binary: php .oracle/pint/builds/pint (the committed PHAR Pinkary runs).
#   .oracle/php-cs-fixer  PHP-CS-Fixer source + tests at the exact version Pint bundles,
#                         with dev deps (PHPUnit) for fixture export. Binary: php .oracle/php-cs-fixer/php-cs-fixer
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
PINT_REF=cef1b260c611f92e66902d678092e26932c9ea00
FIXER_REF=0ee88422118f3cc59c8c3def222ba7f1493b6d5b # v3.95.17

fetch() { # dir url ref
  if [ "$(git -C "$1" rev-parse HEAD 2>/dev/null)" = "$3" ]; then return; fi
  rm -rf "$1" && git init -q "$1"
  git -C "$1" fetch -q --depth 1 "$2" "$3"
  git -C "$1" -c advice.detachedHead=false checkout -q FETCH_HEAD
}

mkdir -p "$root/.oracle"
fetch "$root/.oracle/pint" https://github.com/laravel/pint.git "$PINT_REF"
fetch "$root/.oracle/php-cs-fixer" https://github.com/PHP-CS-Fixer/PHP-CS-Fixer.git "$FIXER_REF"
(cd "$root/.oracle/pint" && composer install -q --no-interaction --no-progress)
(cd "$root/.oracle/php-cs-fixer" && composer install -q --no-interaction --no-progress)

# PHPStan oracle (pun-phpstan): pinned by the committed lock file.
mkdir -p "$root/.oracle/phpstan"
cp "$root/scripts/oracle-phpstan/composer.json" "$root/scripts/oracle-phpstan/composer.lock" "$root/.oracle/phpstan/"
(cd "$root/.oracle/phpstan" && composer install -q --no-interaction --no-progress)

php "$root/.oracle/pint/builds/pint" --version
php "$root/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar" --version
php "$root/.oracle/php-cs-fixer/php-cs-fixer" --version
bundled=$(php -r 'Phar::loadPhar($argv[1], "p.phar"); foreach (json_decode(file_get_contents("phar://p.phar/vendor/composer/installed.json"), true)["packages"] as $p) if ($p["name"] === "friendsofphp/php-cs-fixer") echo $p["source"]["reference"];' "$root/.oracle/pint/builds/pint")
[ "$bundled" = "$FIXER_REF" ] || { echo "PHAR bundles php-cs-fixer $bundled, expected $FIXER_REF" >&2; exit 1; }
echo "oracle ok"
