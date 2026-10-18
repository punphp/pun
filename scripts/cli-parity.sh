#!/bin/bash
# Compare `pint` (the pinned PHAR) with `pun migrate` + `pun format`: exit code and resulting files for each argument
# set, and stdout/stderr byte for byte when the set picks a machine format (`--format=...`). Human output is Pun's own
# (context/plan/output.md), so it is not compared. Both run in the same directory (so absolute paths in diffs / ERROR
# lines agree).
#
#   scripts/cli-parity.sh [--project DIR] [--tty] [--env K=V]... ["ARGS"...]
#
# Without argument sets it runs the default ones below (human runs, every machine format, -v diffs).
# --project  copy of this project is used (default: a small sample with a fixable, a clean and a broken file).
#            node_modules (for Pint/laravel_blade) is symlinked, .git is dropped.
# --tty      run both under `script` so stdout/stderr are a terminal (colours, stty width); output is merged.
# --cols N   with --tty: no COLUMNS, the pty is N columns wide (exercises the `stty -a` width lookup).
# --env      extra environment (both tools run under `env -i PATH HOME`, so agent detection stays off).
# Exit 1 if any argument set differs. `-vvv`: Pint's PHAR prints a "Box Requirements Checker" block that is not
# part of Pint; it is stripped before comparing.
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
project="" tty=0 cols="" envs=(COLUMNS=80)
while [ $# -gt 0 ]; do
  case "$1" in
    --project) project=$(cd "$2" && pwd); shift 2 ;;
    --tty) tty=1; shift ;;
    --cols) cols=$2; envs=(); shift 2 ;;
    --env) envs+=("$2"); shift 2 ;;
    *) break ;;
  esac
done
W=$(mktemp -d)
trap 'rm -rf "$W"' EXIT
src=$W/src
if [ -z "$project" ]; then
  mkdir -p "$src"
  printf '{"preset":"laravel"}' > "$src/pint.json"
  printf '<?php\n$a = array( 1,2 );\nif($a){echo "x";}\n' > "$src/a.php"
  printf '<?php\n\necho 1;\n' > "$src/ok.php"
  printf '<?php\nfunction (\n' > "$src/bad.php"
else
  rsync -a --exclude=.git --exclude=node_modules "$project/" "$src/"
  [ -d "$project/node_modules" ] && ln -s "$project/node_modules" "$src/node_modules"
fi
# Pun's side: `pun migrate` on the same pint.json, then `pun format` with the same arguments.
mkdir -p "$W/bin" && printf '#!/bin/sh\n"%s" migrate >/dev/null 2>&1; exec "%s" format "$@"\n' "$R/target/debug/pun" "$R/target/debug/pun" > "$W/bin/pint" && chmod +x "$W/bin/pint"
P=$W/proj
status=0
run() { # name cmd...
  rm -rf "$P" && cp -a "$src" "$P"
  local out=$W/$1; shift
  if [ $tty = 1 ]; then
    (cd "$P" && env -i PATH="$PATH" HOME="$HOME" TERM=xterm "${envs[@]}" script -qefc "${cols:+stty cols $cols rows 40; }$(printf '%q ' "$@")" /dev/null > "$out.out" 2>&1; echo "exit=$?" > "$out.code")
    : > "$out.err"
  else
    (cd "$P" && env -i PATH="$PATH" HOME="$HOME" "${envs[@]}" "$@" > "$out.out" 2> "$out.err"; echo "exit=$?" > "$out.code")
  fi
  # Box's requirements checker (PHAR wrapper, -vvv only) is not Pint output.
  perl -0pi -e 's/\A\r?\nBox Requirements Checker\r?\n.*?\[OK\] Your system is ready[^\n]*\n *\r?\n\r?\n//s' "$out.out" "$out.err"
  (cd "$P" && find . -path ./node_modules -prune -o -type f ! -name pun.toml -print0 | sort -z | xargs -0 md5sum) > "$out.files"
}
if [ $# -eq 0 ]; then
  set -- "" "--test" "-v" "--test -v" "--repair" "--bail" \
    "--format=json" "--test --format=json -v" "--test --format=txt -v" "--test --format=checkstyle" \
    "--test --format=junit -v" "--test --format=xml -v" "--test --format=gitlab" "--test --format=agent" \
    "--test --output-to-file=r.json --output-format=json"
fi
for args in "$@"; do
  read -ra a <<< "$args"
  # Machine formats are compared byte for byte; human stdout/stderr are Pun's own.
  keys="code files"
  case " $args " in *" --format"*) keys="out err code files" ;; esac
  run pint php "$R/.oracle/pint/builds/pint" ${a[@]+"${a[@]}"}
  run pun "$W/bin/pint" ${a[@]+"${a[@]}"}
  same=1
  for k in $keys; do
    if ! cmp -s "$W/pint.$k" "$W/pun.$k"; then
      [ $same = 1 ] && echo "##### $args: DIFFERS"
      same=0
      echo "--- $k (< pint, > pun)"
      diff <(cat -v "$W/pint.$k") <(cat -v "$W/pun.$k") | head -40
    fi
  done
  [ -n "${SHOW:-}" ] && cat -v "$W/pun.out"
  if [ $same = 1 ]; then echo "##### $args: identical"; else status=1; fi
done
exit $status
