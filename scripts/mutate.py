#!/usr/bin/env python3
"""Style mutations for scripts/fuzz-parity.sh.

  mutate.py mutate <dir> <seed>            mutate every non-Blade .php file under <dir> in place (3 mutations each);
                                           deterministic per (seed, relative path), so subsets agree with full runs;
                                           FUZZ_EXTRA=crlf,mixed-eol also applies those mutations to every file, last
  mutate.py minimise <file> <project-dir>  shrink <file> (lines, then characters) while Pint and Pun still
                                           disagree on it, with <project-dir>'s pint.json; prints the result

Mutations only touch style (whitespace, case, syntax variants PHP treats alike), so every mutant should still
format; a mutation that breaks the code is fine too (both tools must then report the same syntax error).
"""
import os, random, re, shutil, subprocess, sys, tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def shuffle_imports(s, rnd):
    def shuffle(m):
        lines = m.group(0).splitlines(keepends=True)
        rnd.shuffle(lines)
        return ''.join(lines)
    return re.sub(r'(?m)(?:^use [^;\n(]+;\n){2,}', shuffle, s)


def quotes(s, rnd):
    # 'simple' <-> "simple" (no $, backslash or quotes inside, so the value is unchanged).
    if rnd.random() < 0.5:
        return re.sub(r"'([^'\"\\$\n{}]*)'", r'"\1"', s)
    return re.sub(r'"([^\'"\\$\n{}]*)"', r"'\1'", s)


MUTATIONS = [
    ('dedent', lambda s, r: re.sub(r'(?m)^[ \t]+', '', s)),
    ('tabs', lambda s, r: re.sub(r'(?m)^(    )+', lambda m: '\t' * (len(m.group(0)) // 4), s)),
    ('two-space-indent', lambda s, r: re.sub(r'(?m)^((?:    )+)', lambda m: '  ' * (len(m.group(1)) // 4), s)),
    ('trailing-ws', lambda s, r: re.sub(r'(?m)$', '  ', s)),
    ('paren-space', lambda s, r: s.replace('(', '( ').replace(')', ' )')),
    ('comma-nospace', lambda s, r: s.replace(', ', ',')),
    ('comma-space', lambda s, r: re.sub(r',(?=\S)', ' , ', s)),
    ('upper-kw', lambda s, r: re.sub(r'\b(return|function|if|else|foreach|new|true|false|null|static|self|array|fn)\b', lambda m: m.group(1).upper(), s)),
    ('extra-blank', lambda s, r: re.sub(r'\n\n', '\n\n\n\n', s)),
    ('no-blank', lambda s, r: re.sub(r'\n\s*\n', '\n', s)),
    ('brace-newline', lambda s, r: re.sub(r'\)\s*\{', ')\n{', s)),
    ('brace-same-line', lambda s, r: re.sub(r'\)\s*\n\s*\{', ') {', s)),
    ('concat-space', lambda s, r: re.sub(r"\s*\.\s*(?=['$])", ' . ', s)),
    ('concat-nospace', lambda s, r: re.sub(r" \. ", '.', s)),
    ('long-array', lambda s, r: re.sub(r'= \[\]', '= array()', s)),
    ('crlf', lambda s, r: s.replace('\n', '\r\n')),
    ('mixed-eol', lambda s, r: re.sub('\n', lambda m: r.choice(('\n', '\r\n', '\r')), s)),
    ('doc-type-alias', lambda s, r: re.sub(r'(@(?:param|return|var)\s+\S*?)\b(int|bool|float|string)\b', lambda m: m.group(1) + {'int': 'integer', 'bool': 'boolean', 'float': 'double', 'string': 'str'}[m.group(2)], s)),
    ('doc-type-case', lambda s, r: re.sub(r'(@(?:param|return|var)\s+\S*?)\b(string|int|array|null|bool|mixed|void)\b', lambda m: m.group(1) + m.group(2).capitalize(), s)),
    ('shuffle-imports', shuffle_imports),
    ('quotes', quotes),
    ('op-nospace', lambda s, r: re.sub(r' (===|!==|==|!=|<=|>=|&&|\|\||\?\?|=>|\+=|-=|\.=) ', r'\1', s)),
    ('op-wide', lambda s, r: re.sub(r' (===|!==|=|=>|\?\?) ', r'   \1  ', s)),
    ('arrow-space', lambda s, r: re.sub(r'(?<=\w|\))->(?=\w)', ' -> ', s)),
    ('cast-space', lambda s, r: re.sub(r'\((int|string|bool|float|array|object)\)\s*', r'( \1 ) ', s)),
    ('cast-long', lambda s, r: re.sub(r'\((int|bool)\)', lambda m: {'int': '(integer)', 'bool': '(boolean)'}[m.group(1)], s)),
    ('docblock-squash', lambda s, r: re.sub(r'(?m)^(\s*)\* ', r'\1*', s)),
    ('docblock-blank', lambda s, r: re.sub(r'/\*\*\n', '/**\n *\n', s)),
    ('else-if', lambda s, r: re.sub(r'\belseif\b', 'else if', s)),
    ('elseif', lambda s, r: re.sub(r'\belse if\b', 'elseif', s)),
    ('no-strict-types', lambda s, r: re.sub(r'declare\(strict_types=1\);\n*', '', s)),
    ('strict-types-spaced', lambda s, r: s.replace('declare(strict_types=1);', 'declare( strict_types = 1 );')),
    ('no-visibility', lambda s, r: re.sub(r'(?m)^(\s+)public (function|static function)\b', r'\1\2', s)),
    ('var-property', lambda s, r: re.sub(r'(?m)^(\s+)public (\$\w+)', r'\1var \2', s)),
    ('semicolon-space', lambda s, r: re.sub(r';\n', ' ;\n', s)),
    ('nullable-space', lambda s, r: re.sub(r'\?(?=[A-Z]\w*\s+\$)', '? ', s)),
    ('return-type-space', lambda s, r: re.sub(r'\): ', ') :', s)),
    ('no-final-newline', lambda s, r: s.rstrip('\n')),
    # laravel-framework's own rules: phpdoc_summary, phpdoc_return_self_reference, phpdoc_trim_consecutive_blank_line_separation,
    # combine_consecutive_issets / _unsets, class_reference_name_casing, switch_continue_to_break.
    ('doc-summary-nodot', lambda s, r: re.sub(r'(?m)^(\s*/\*\*\n\s*\* [^@\n]*?)\.$', r'\1', s)),
    ('doc-return-this', lambda s, r: re.sub(r'@return (\$this|static|self)\b', lambda m: '@return ' + {'$this': 'this', 'static': '@static', 'self': '$self'}[m.group(1)], s)),
    ('doc-blank-run', lambda s, r: re.sub(r'(?m)^([ \t]*\*)\n', r'\1\n\1\n\1\n', s)),
    ('split-isset', lambda s, r: re.sub(r'isset\(([^(),]+), ([^(),]+)\)', r'isset(\1) && isset(\2)', s)),
    ('split-unset', lambda s, r: re.sub(r'unset\(([^(),;]+), ([^(),;]+)\);', r'unset(\1); unset(\2);', s)),
    ('class-case', lambda s, r: re.sub(r'\\(Exception|InvalidArgumentException|RuntimeException|LogicException|stdClass|Closure|ArrayIterator|DateTime|DateTimeImmutable|Throwable)\b', lambda m: '\\' + m.group(1).lower(), s)),
    ('break-continue', lambda s, r: re.sub(r'\bbreak;', 'continue;', s)),
]


def mutate(root, seed):
    applied = {}
    for d, _, fs in sorted(os.walk(root)):
        for f in sorted(fs):
            if not f.endswith('.php') or f.endswith('.blade.php'):
                continue
            p = os.path.join(d, f)
            rnd = random.Random(f'{seed}:{os.path.relpath(p, root)}')
            s = open(p, encoding='utf-8', errors='surrogateescape', newline='').read()
            extra = [(n, fn) for n, fn in MUTATIONS if n in os.environ.get('FUZZ_EXTRA', '').split(',')]
            for name, fn in rnd.sample(MUTATIONS, 3) + extra:
                s = fn(s, rnd)
                applied[name] = applied.get(name, 0) + 1
            open(p, 'w', encoding='utf-8', errors='surrogateescape', newline='').write(s)
    print(' '.join(f'{k}={v}' for k, v in sorted(applied.items())))


def outputs(code, project):
    """(Pint's file, Pun's file) after formatting `code` alone with the project's config."""
    res = []
    for tool in ('pint', 'pun'):
        d = tempfile.mkdtemp()
        for cfg in ('pint.json', 'package.json'):
            if os.path.exists(os.path.join(project, cfg)):
                shutil.copy(os.path.join(project, cfg), d)
        if os.path.exists(os.path.join(project, 'node_modules')):
            os.symlink(os.path.realpath(os.path.join(project, 'node_modules')), os.path.join(d, 'node_modules'))
        open(os.path.join(d, 'x.php'), 'wb').write(code)
        cmd = ['php', os.path.join(ROOT, '.oracle/pint/builds/pint'), 'x.php'] if tool == 'pint' else [os.environ.get('PUN_BIN', os.path.join(ROOT, 'target/debug/pun')), 'format', 'x.php']
        subprocess.run(cmd, cwd=d, capture_output=True, env={'PATH': os.environ['PATH'], 'HOME': os.environ.get('HOME', '/tmp')}, timeout=300)
        res.append(open(os.path.join(d, 'x.php'), 'rb').read())
        shutil.rmtree(d)
    return res


def minimise(path, project, budget=120):
    code = open(path, 'rb').read()
    calls = 0

    def differs(c):
        nonlocal calls
        calls += 1
        a, b = outputs(c, project)
        return a != b

    for unit in ('lines', 'chars'):
        parts = code.splitlines(keepends=True) if unit == 'lines' else [bytes([b]) for b in code]
        n = 2
        while len(parts) >= 2 and calls < budget:
            chunk = max(1, len(parts) // n)
            for i in range(0, len(parts), chunk):
                cand = parts[:i] + parts[i + chunk:]
                if calls < budget and differs(b''.join(cand)):
                    parts = cand
                    n = max(n - 1, 2)
                    break
            else:
                if chunk == 1:
                    break
                n = min(n * 2, len(parts))
        code = b''.join(parts)
        if unit == 'lines' and len(code) > 400:
            break  # character pass only pays off on small cases
    pint, pun = outputs(code, project)
    sys.stdout.write(f'--- minimised input ({len(code)} bytes, {calls} runs) ---\n{code.decode(errors="replace")}\n'
                     f'--- pint ---\n{pint.decode(errors="replace")}\n--- pun ---\n{pun.decode(errors="replace")}\n')


if __name__ == '__main__':
    if sys.argv[1] == 'mutate':
        mutate(sys.argv[2], int(sys.argv[3]))
    else:
        minimise(sys.argv[2], sys.argv[3])
