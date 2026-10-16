<?php

// Make a deliberately unformatted copy of a PHP tree, in place:
//   php scripts/perturb.php <dir>
//
// 1. PHP-CS-Fixer with scripts/perturb-config.php (anti-laravel structural style).
// 2. Seeded token-level noise: whitespace, keyword/constant/cast casing, quotes, `else if`,
//    `<>`, `#` comments, dropped `final` / `declare(strict_types=1)`.
// Every mutated file must still pass `php -l`, otherwise it is left as step 1 produced it.

$dir = realpath($argv[1]);
$root = dirname(__DIR__);
passthru('PERTURB_DIR='.escapeshellarg($dir).' php '.escapeshellarg("$root/.oracle/php-cs-fixer/php-cs-fixer")
    .' fix -q --config '.escapeshellarg("$root/scripts/perturb-config.php"));

$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($dir, FilesystemIterator::SKIP_DOTS));
$changed = $rejected = 0;
foreach ($it as $f) {
    $path = $f->getPathname();
    if (! str_ends_with($path, '.php') || str_ends_with($path, '.blade.php') || preg_match('#/(vendor|node_modules|\.git)/#', $path)) {
        continue;
    }
    $code = file_get_contents($path);
    mt_srand(crc32(substr($path, strlen($dir))));
    $out = perturb($code);
    if ($out === $code) {
        continue;
    }
    file_put_contents($path, $out);
    exec('php -l '.escapeshellarg($path).' 2>&1', $_, $rc);
    if ($rc !== 0) {
        file_put_contents($path, $code);
        $rejected++;
    } else {
        $changed++;
    }
}
fwrite(STDERR, "perturbed $changed files ($rejected rejected by php -l)\n");

function chance(int $pct): bool
{
    return mt_rand(1, 100) <= $pct;
}

function perturb(string $code): string
{
    $tokens = token_get_all($code);
    $inString = false;
    $out = '';
    $prevText = '';
    foreach ($tokens as $i => $t) {
        [$id, $text] = is_array($t) ? [$t[0], $t[1]] : [null, $t];
        if ($text === '"' || $text === '`' || $id === T_START_HEREDOC || $id === T_END_HEREDOC) {
            $inString = ($text === '"' || $text === '`') ? ! $inString : $id === T_START_HEREDOC;
        }
        $next = $tokens[$i + 1] ?? null;
        $nextId = is_array($next) ? $next[0] : null;

        if (! $inString && $id !== T_INLINE_HTML) {
            $text = match (true) {
                $id === T_WHITESPACE && str_contains($text, "\n") => chance(15) ? preg_replace('/\n/', "  \n", $text, 1)
                    : (chance(10) ? "\n".$text : (chance(10) ? str_replace('    ', "\t", $text) : $text)),
                $id === T_WHITESPACE && $text === ' ' && chance(15) => '  ',
                in_array($id, [T_FUNCTION, T_RETURN, T_IF, T_FOREACH, T_NEW, T_ECHO, T_ARRAY, T_STATIC, T_PUBLIC, T_PRIVATE], true) && chance(20) => strtoupper($text),
                $id === T_STRING && in_array(strtolower($text), ['true', 'false', 'null'], true) && chance(30) => strtoupper($text),
                in_array($id, [T_INT_CAST, T_BOOL_CAST], true) && chance(50) => str_replace(['int', 'bool'], ['INTEGER', 'Boolean'], $text),
                in_array($id, [T_DIR, T_CLASS_C, T_FUNC_C], true) && chance(50) => strtolower($text),
                $id === T_CONSTANT_ENCAPSED_STRING && $text[0] === "'" && ! preg_match('/[$\\\\"]/', $text) && chance(30) => '"'.substr($text, 1, -1).'"',
                $id === T_ELSEIF && chance(50) => 'else if',
                $id === T_IS_NOT_EQUAL && $text === '!=' && chance(50) => '<>',
                $id === T_COMMENT && str_starts_with($text, '// ') && chance(30) => '#'.substr($text, 2),
                $id === T_FINAL && $nextId === T_WHITESPACE && chance(50) => '',
                default => $text,
            };
            if ($id === T_FINAL && $text === '') {
                $tokens[$i + 1] = '';
            }
            // Space between two adjacent non-whitespace tokens (never around `$`, strings or tags).
            if ($prevText !== '' && $id !== T_WHITESPACE && ! ctype_space($prevText[-1]) && $text !== ''
                && $text !== '$' && $prevText !== '$' && ! in_array($id, [T_OPEN_TAG, T_CLOSE_TAG, T_ENCAPSED_AND_WHITESPACE], true)
                && ! str_ends_with($prevText, '<?php') && chance(8)) {
                $text = ' '.$text;
            }
        }
        $out .= $text;
        $prevText = $text !== '' ? $text : $prevText;
    }
    $out = preg_replace('/^(<\?php\s*)declare\(strict_types=1\);\n/', '$1', $out, 1);

    return chance(20) ? rtrim($out)."\n\n\n" : $out;
}
