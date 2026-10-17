#!/usr/bin/env python3
"""Level-0 corpus parity for `pun analyse` (Phase 2 M2.3).

    scripts/analyse-corpus.py setup  [project...]   copy corpus/<p> to target-analyse/corpus/<p>, composer install,
                                                    pin PHPStan 2.2.7 (symlink to .oracle/phpstan)
    scripts/analyse-corpus.py oracle [project...]   phpstan analyse --level 0 --error-format=json (cold, no result
                                                    cache) -> target-analyse/oracle/<p>.json (+ seconds)
    scripts/analyse-corpus.py diff   [project...]   run Pun's analyser (target-analyse/debug/examples/analyse) and diff
                                                    (file, line, identifier, message) per identifier
    scripts/analyse-corpus.py rank                  error count per identifier over the oracle runs
    scripts/analyse-corpus.py fixture               PHPStan's errors on crates/pun-analyse/tests/fixtures/project for
                                                    each of its phpstan*.neon -> expected-<config>.json (relative paths)

The configs are the projects' own includes and paths with `level: 0` and without ignoreErrors and baselines
(ignores are M2.1, tested on their own), written to <copy>/pun-level0.neon.
"""
import collections, json, os, shutil, subprocess, sys, time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WORK = os.path.join(ROOT, 'target-analyse')

# project -> (includes, extra parameters as NEON lines)
PROJECTS = {
    'pinkary': (['vendor/larastan/larastan/extension.neon', 'vendor/phpstan/phpstan/conf/bleedingEdge.neon'],
                ['paths: [app, config, bootstrap, database/factories, routes]']),
    'larastan': (['vendor/phpstan/phpstan/conf/bleedingEdge.neon', 'vendor/phpstan/phpstan-deprecation-rules/rules.neon',
                  'extension.neon'],
                 ['phpVersion: 80200', 'paths: [src]']),
    'laravel-framework': ([], ['paths: [src]',
                               "excludePaths: [src/Illuminate/Testing/ParallelRunner.php, 'src/*/views/*']"]),
    'pest': (['phpstan-pest-extension.neon', 'vendor/pestphp/pest-plugin-phpstan/extension.neon'], ['paths: [src]']),
}


def copy_dir(p):
    return os.path.join(WORK, 'corpus', p)


# PHPStan 2.2 switches to its agent output format when it sees one of these (src/Internal/AgentDetector.php).
AGENT_ENV = ['AUGMENT_AGENT', 'AMP_CURRENT_THREAD_ID', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT', 'GEMINI_CLI',
             'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE', 'CLAUDE_CODE', 'REPL_ID']


def run(cmd, cwd, env=None):
    env = {k: v for k, v in (env or os.environ).items() if k not in AGENT_ENV}
    # php.ini's max_memory_limit (PHP 8.5) caps --memory-limit, also in PHPStan's worker processes, which don't get
    # our -d flags: raise it through an extra scan dir.
    ini = os.path.join(WORK, 'php-ini')
    os.makedirs(ini, exist_ok=True)
    with open(os.path.join(ini, 'pun.ini'), 'w') as f:
        f.write('max_memory_limit = -1\n')
    env['PHP_INI_SCAN_DIR'] = ':' + ini
    return subprocess.run(['nice', 'timeout', '3600'] + cmd, cwd=cwd, env=env, capture_output=True, text=True)


# The corpus larastan commit needs PHPStan 2.3; the oracle pins Larastan v3.10.0, so analyse that tag.
TAGS = {'larastan': ('https://github.com/larastan/larastan.git', 'v3.10.0')}


def setup(p):
    dst = copy_dir(p)
    if p in TAGS:
        shutil.rmtree(dst, ignore_errors=True)
        subprocess.run(['git', 'clone', '-q', '--depth', '1', '-b', TAGS[p][1], TAGS[p][0], dst], check=True)
        shutil.rmtree(os.path.join(dst, '.git'))
    os.makedirs(dst, exist_ok=True)
    p in TAGS or subprocess.run(['rsync', '-a', '--exclude', '.git', '--exclude', 'node_modules', '--exclude', 'vendor',
                    os.path.join(ROOT, 'corpus', p) + '/', dst + '/'], check=True)
    env = dict(os.environ, COMPOSER_ROOT_VERSION='5.99.0') if p == 'pest' else None
    r = run(['composer', 'install', '--no-scripts', '--no-interaction', '--no-progress', '--ignore-platform-reqs', '-q'],
            dst, env)
    if r.returncode:
        sys.exit(r.stderr)
    # PHPStan and its deprecation rules at the oracle's versions (a newer phpstan-deprecation-rules needs 2.3)
    for pkg in ['phpstan', 'phpstan-deprecation-rules']:
        pinned = os.path.join(dst, 'vendor/phpstan', pkg)
        if os.path.isdir(pinned) and not os.path.islink(pinned):
            shutil.rmtree(pinned)
            os.symlink(os.path.join(ROOT, '.oracle/phpstan/vendor/phpstan', pkg), pinned)
    write_config(p)


def write_config(p):
    includes, params = PROJECTS[p]
    lines = ['includes:'] + [f'    - {i}' for i in includes] if includes else []
    # One worker: the box is shared (2 cores / 7 GB), and the timing compares one process with one process.
    lines += ['parameters:', '    level: 0', '    tmpDir: ' + os.path.join(WORK, 'tmp', 'phpstan-' + p),
              '    parallel:', '        maximumNumberOfProcesses: 1']
    lines += ['    ' + l for l in params]
    with open(os.path.join(copy_dir(p), 'pun-level0.neon'), 'w') as f:
        f.write('\n'.join(lines) + '\n')


def oracle(p):
    write_config(p)
    d = copy_dir(p)
    shutil.rmtree(os.path.join(WORK, 'tmp', 'phpstan-' + p), ignore_errors=True)
    t = time.monotonic()
    r = run(['php', 'vendor/phpstan/phpstan/phpstan.phar', 'analyse', '-c', 'pun-level0.neon', '--error-format=json',
             '--no-progress', '--memory-limit=' + os.environ.get('PHPSTAN_MEMORY', '1G')], d)
    secs = time.monotonic() - t
    try:
        data = json.loads(r.stdout)
    except json.JSONDecodeError:
        sys.exit(f'{p}: phpstan exit {r.returncode}\n{r.stdout[-2000:]}\n{r.stderr[-2000:]}')
    errors = []
    for file, fe in data['files'].items():
        for m in fe['messages']:
            errors.append({'file': os.path.relpath(file, d), 'line': m.get('line'), 'identifier': m.get('identifier'),
                           'message': m['message'], 'tip': m.get('tip')})
    errors.sort(key=lambda e: (e['file'], e['line'] or 0, e['identifier'] or '', e['message']))
    out = {'seconds': round(secs, 2), 'general': data['errors'], 'errors': errors}
    os.makedirs(os.path.join(WORK, 'oracle'), exist_ok=True)
    with open(os.path.join(WORK, 'oracle', p + '.json'), 'w') as f:
        json.dump(out, f, indent=1)
    print(f'{p}: {len(errors)} errors, {len(data["errors"])} general, {secs:.1f}s')


FIXTURE = os.path.join(ROOT, 'crates/pun-analyse/tests/fixtures/project')


def fixture():
    for neon in sorted(f for f in os.listdir(FIXTURE) if f.startswith('phpstan') and f.endswith('.neon')):
        tmp = os.path.join(WORK, 'tmp', 'fixture-project')
        shutil.rmtree(tmp, ignore_errors=True)
        os.makedirs(tmp)
        wrapper = os.path.join(tmp, 'wrapper.neon')
        with open(wrapper, 'w') as f:
            f.write(f'includes:\n    - {os.path.join(FIXTURE, neon)}\nparameters:\n    tmpDir: {tmp}\n')
        r = run(['php', os.path.join(ROOT, '.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar'), 'analyse', '-c',
                 wrapper, '--error-format=json', '--no-progress'], FIXTURE)
        # path-independent: the checkout's path is {root} in messages
        data = json.loads(r.stdout.replace(json.dumps(FIXTURE)[1:-1], '{root}'))
        errors = [{'file': file.removeprefix('{root}/'), 'line': m.get('line'), 'identifier': m.get('identifier'),
                   'message': m['message'], 'tip': m.get('tip')}
                  for file, fe in data['files'].items() for m in fe['messages']]
        errors.sort(key=lambda e: (e['file'], e['line'] or 0, e['identifier'] or '', e['message']))
        out = os.path.join(FIXTURE, 'expected-' + neon[:-5] + '.json')
        with open(out, 'w') as f:
            json.dump({'general': data['errors'], 'errors': errors}, f, indent=1)
            f.write('\n')
        print(f'{neon}: {len(errors)} errors')


def load(p):
    with open(os.path.join(WORK, 'oracle', p + '.json')) as f:
        return json.load(f)


def rank(projects):
    c = collections.Counter()
    for p in projects:
        for e in load(p)['errors']:
            c[e['identifier']] += 1
    for ident, n in c.most_common():
        print(f'{n:6} {ident}')


def key(e):
    return (e['file'], e['line'], e['identifier'], e['message'])


def diff(p):
    want = load(p)
    exe = os.path.join(WORK, 'debug/examples/analyse')
    d = copy_dir(p)
    t = time.monotonic()
    r = run([exe, d, os.path.join(d, 'pun-level0.neon')], d)
    secs = time.monotonic() - t
    if r.returncode:
        sys.exit(r.stderr)
    got = json.loads(r.stdout)
    ported = set(got['rules'])
    w = {key(e) for e in want['errors'] if e['identifier'] in ported}
    g = {key(e) for e in got['errors']}
    per = collections.defaultdict(lambda: [0, 0, 0])
    for k in w & g:
        per[k[2]][0] += 1
    for k in w - g:
        per[k[2]][1] += 1
    for k in g - w:
        per[k[2]][2] += 1
    print(f'{p}: pun {secs:.2f}s vs phpstan {want["seconds"]}s ({want["seconds"] / secs:.1f}x); '
          f'missing {len(w - g)}, extra {len(g - w)}')
    for ident in sorted(per):
        same, miss, extra = per[ident]
        print(f'  {ident:40} same {same:5} missing {miss:5} extra {extra:5}')
    for k in sorted(w - g)[:int(os.environ.get('DIFFS', '10'))]:
        print('  - ', *k)
    for k in sorted(g - w)[:int(os.environ.get('DIFFS', '10'))]:
        print('  + ', *k)
    return not (w ^ g)


def main():
    cmd, names = sys.argv[1], sys.argv[2:] or list(PROJECTS)
    if cmd == 'rank':
        return rank(names)
    if cmd == 'fixture':
        return fixture()
    ok = True
    for p in names:
        ok &= {'setup': setup, 'oracle': oracle, 'diff': diff}[cmd](p) is not False
    sys.exit(0 if ok else 1)


if __name__ == '__main__':
    main()
