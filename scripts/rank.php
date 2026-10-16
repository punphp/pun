<?php

// Count, per fixer, how many files each fixer changes when run alone.
//
//   php scripts/rank.php <pint.json> <dir>...  > counts.json
//
// Fixers are configured from <pint.json>'s effective rule list (see pint-boot.php);
// files come from Pint's own finder rooted at each <dir> (Blade files skipped).
// Mirrors PHP-CS-Fixer's Runner per file: supports() -> isCandidate() -> fix() ->
// clearEmptyTokens(), on a fresh clone of the file's token list for every fixer.

$dirs = array_slice($argv, 2);
$argv = [$argv[0], $argv[1]];
[$pint, $config, $factory] = require __DIR__.'/pint-boot.php';

use PhpCsFixer\Tokenizer\Tokens;

$fixers = array_filter($factory->getFixers(), fn ($f) => $f->getName() !== 'Pint/laravel_blade');
$counts = [];
foreach ($dirs as $dir) {
    $n = array_fill_keys(array_map(fn ($f) => $f->getName(), $fixers), 0);
    $files = 0;
    foreach (App\Factories\ConfigurationFactory::finder()->in($dir)->notName('*.blade.php') as $file) {
        $code = file_get_contents($file->getPathname());
        Tokens::clearCache();
        try {
            $pristine = Tokens::fromCode($code);
        } catch (ParseError) {
            continue; // Pint's linter skips files that don't parse.
        }
        $files++;
        foreach ($fixers as $fixer) {
            if (! $fixer->supports($file)) {
                continue;
            }
            $tokens = clone $pristine;
            if (! $fixer->isCandidate($tokens)) {
                continue;
            }
            $fixer->fix($file, $tokens);
            $tokens->clearEmptyTokens();
            $n[$fixer->getName()] += (int) ($tokens->generateCode() !== $code);
        }
    }
    fwrite(STDERR, basename($dir).": $files files\n");
    foreach ($n as $name => $c) {
        $counts[$name][basename($dir)] = $c;
    }
}
echo json_encode($counts, JSON_PRETTY_PRINT)."\n";
