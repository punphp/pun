#!/usr/bin/env python3
"""PHPStan + Larastan's types for the Laravel fixture (crates/pun-analyse/tests/laravel.rs).

    scripts/export-laravel-fixture.py <pinkary copy>

It needs a copy of corpus/pinkary with `composer install` done (never the corpus checkout itself). It copies
crates/pun-analyse/tests/fixtures/laravel/app (all but Pinkary's own Link and User and its AppServiceProvider, which
the copy already has) into it, runs `phpstan analyse` on app/Probe.php with the fixture's migrations, and writes the
dumped types to tests/fixtures/laravel/expected.json, for Larastan's parseModelCastsMethod false and true.
"""
import glob, json, os, shutil, subprocess, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

FIXTURE = os.path.join(ROOT, 'crates/pun-analyse/tests/fixtures/laravel')
PINKARY_OWN = {'app/Models/Link.php', 'app/Models/User.php', 'app/Providers/AppServiceProvider.php'}


def oracle():
    copy = os.path.abspath(sys.argv[1])
    for f in glob.glob(os.path.join(FIXTURE, 'app/**/*.php'), recursive=True):
        rel = os.path.relpath(f, FIXTURE)
        if rel not in PINKARY_OWN:
            os.makedirs(os.path.dirname(os.path.join(copy, rel)), exist_ok=True)
            shutil.copy(f, os.path.join(copy, rel))
    out = {}
    for parse in ['false', 'true']:
        tmp = os.path.join(copy, 'tmp-probe-' + parse)
        shutil.rmtree(tmp, ignore_errors=True)
        neon = os.path.join(copy, 'probe.neon')
        with open(neon, 'w') as f:
            f.write(f"includes:\n    - vendor/larastan/larastan/extension.neon\nparameters:\n    level: 0\n"
                    f"    tmpDir: {tmp}\n    paths: [app/Probe.php]\n    parseModelCastsMethod: {parse}\n"
                    f"    databaseMigrationsPath: [{os.path.join(FIXTURE, 'database/migrations')}]\n")
        # PHPStan's AgentDetector prints agent instructions before the JSON under these (as export-analyse-expected.py).
        agents = {'AUGMENT_AGENT', 'AMP_CURRENT_THREAD_ID', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT', 'GEMINI_CLI',
                  'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE', 'CLAUDE_CODE', 'REPL_ID'}
        env = {k: v for k, v in os.environ.items() if k not in agents}
        r = subprocess.run(['php', 'vendor/phpstan/phpstan/phpstan.phar', 'analyse', '-c', neon, '--error-format=json',
                            '--no-progress'], cwd=copy, env=env, capture_output=True, text=True)
        data = json.loads(r.stdout)
        out['parseModelCastsMethod=' + parse] = sorted(
            [m['line'], m['message'].removeprefix('Dumped type: ')]
            for fe in data['files'].values() for m in fe['messages'] if m['message'].startswith('Dumped type: '))
    with open(os.path.join(FIXTURE, 'expected.json'), 'w') as f:
        json.dump(out, f, indent=1)
        f.write('\n')
    print({k: len(v) for k, v in out.items()})


oracle()
