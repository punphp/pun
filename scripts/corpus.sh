#!/usr/bin/env bash
# Shallow-clone every project in corpus/manifest.toml into corpus/<name> at its pinned commit.
# Usage: scripts/corpus.sh [name...]   (default: all)
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
manifest="$root/corpus/manifest.toml"

awk -F' = ' '/^name/{n=$2} /^url/{u=$2} /^commit/{gsub(/"/,"",n); gsub(/"/,"",u); gsub(/"/,"",$2); print n, u, $2}' "$manifest" |
while read -r name url commit; do
  if [ $# -gt 0 ] && [[ ! " $* " =~ " $name " ]]; then continue; fi
  dir="$root/corpus/$name"
  if [ "$commit" = derived ]; then
    if [ -d "$dir/.git" ]; then echo "ok   $name (derived; rm -rf corpus/$name to rebuild)"; continue; fi
    echo "make $name from $url"
    rm -rf "$dir" && mkdir -p "$dir"
    (cd "$root/corpus/$url" && tar --exclude=./.git --exclude=./node_modules -cf - .) | tar -xf - -C "$dir"
    [ -d "$root/corpus/$url/node_modules" ] && ln -s "../$url/node_modules" "$dir/node_modules"
    php "$root/scripts/perturb.php" "$dir"
    git -C "$dir" init -q && git -C "$dir" add -A && git -C "$dir" -c user.name=pun -c user.email=pun@localhost commit -qm perturbed
    continue
  fi
  if [ "$(git -C "$dir" rev-parse HEAD 2>/dev/null)" = "$commit" ]; then echo "ok   $name $commit"; continue; fi
  echo "get  $name $commit"
  rm -rf "$dir" && git init -q "$dir"
  [ -d "$url" ] && url="file://$url"
  git -C "$dir" fetch -q --depth 1 "$url" "$commit"
  git -C "$dir" -c advice.detachedHead=false checkout -q FETCH_HEAD
  if [ "$name" = pinkary ]; then
    # Pint/laravel_blade needs prettier; Pinkary has no package-lock, so pin what the oracle used.
    (cd "$dir" && npm install -q --no-save --no-audit --no-fund --ignore-scripts \
      prettier@3.9.10 prettier-plugin-blade@3.3.3 prettier-plugin-tailwindcss@0.8.1)
  fi
done
