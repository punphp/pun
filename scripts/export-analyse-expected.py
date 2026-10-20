#!/usr/bin/env python3
"""PHPStan's errors on a pun-analyse fixture directory, as tests/fixtures/<dir>/expected-<neon>.json.

    scripts/export-analyse-expected.py crates/pun-analyse/tests/fixtures/coverage/<case> [phpstan]

Runs .oracle/phpstan from crates/pun-analyse (the working directory of `cargo test`, which names anonymous classes)
with `-c <dir>/<neon>.neon --error-format=json`, and writes the errors with paths relative to <dir>.
"""
import json, os, subprocess, sys

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
d = os.path.abspath(sys.argv[1])
neon = sys.argv[2] if len(sys.argv) > 2 else "phpstan"
# PHPStan's AgentDetector (src/Internal/AgentDetector.php) prints agent instructions before the JSON under these
agents = {"AUGMENT_AGENT", "AMP_CURRENT_THREAD_ID", "AI_AGENT", "CURSOR_TRACE_ID", "CURSOR_AGENT", "GEMINI_CLI", "CODEX_SANDBOX",
          "CODEX_THREAD_ID", "OPENCODE_CLIENT", "OPENCODE", "CLAUDECODE", "CLAUDE_CODE", "REPL_ID"}
env = {**{k: v for k, v in os.environ.items() if k not in agents}, "XDEBUG_MODE": "off"}
out = subprocess.run([os.path.join(root, ".oracle/phpstan/vendor/bin/phpstan"), "analyse", "-c", f"{d}/{neon}.neon",
                      "--error-format=json", "--no-progress"], cwd=os.path.join(root, "crates/pun-analyse"),
                     capture_output=True, text=True, env=env)
data = json.loads(out.stdout)
errors = [{"file": os.path.relpath(f, d), "line": m["line"], "identifier": m.get("identifier"),
           "message": m["message"], "tip": m.get("tip")}
          for f, v in sorted(data["files"].items()) for m in v["messages"]]
json.dump({"general": data["errors"], "errors": errors}, open(f"{d}/expected-{neon}.json", "w"), indent=1)
for e in errors:
    print(f'{e["file"]}:{e["line"]} {e["identifier"]} {e["message"]}')
