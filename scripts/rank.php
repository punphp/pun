<?php

// Count, per fixer, how many files each fixer changes when run alone.
//
//   php scripts/rank.php <pint.json> <dir>...  > counts.json
//
// Fixers are configured from <pint.json>'s effective rule list (see pint-boot.php);
// files come from Pint's own finder rooted at each <dir> (Blade files skipped).
// Mirrors PHP-CS-Fixer's Runner per file: supports() -> isCandidate() -> fix() ->
// clearEmptyTokens(), on a fresh token list for every (fixer, file).

$dirs = array_slice($argv, 2);
$argv = [$argv[0], $argv[1]];
[$pint, $config, $factory] = require __DIR__.'/pint-boot.php';

use PhpCsFixer\Tokenizer\Tokens;

$counts = [];
foreach ($dirs as $dir) {
    $files = [];
    foreach (App\Factories\ConfigurationFactory::finder()->in($dir)->notName('*.blade.php') as $f) {
        $files[] = [$f, file_get_contents($f->getPathname())];
    }
    fwrite(STDERR, basename($dir).': '.count($files)." files\n");
    foreach ($factory->getFixers() as $fixer) {
        if (str_starts_with($fixer->getName(), 'Pint/laravel_blade')) {
            continue;
        }
        $n = 0;
        foreach ($files as [$file, $code]) {
            if (! $fixer->supports($file)) {
                continue;
            }
            Tokens::clearCache();
            $tokens = Tokens::fromCode($code);
            if (! $fixer->isCandidate($tokens)) {
                continue;
            }
            $fixer->fix($file, $tokens);
            $tokens->clearEmptyTokens();
            $n += (int) ($tokens->generateCode() !== $code);
        }
        $counts[$fixer->getName()][basename($dir)] = $n;
    }
}
echo json_encode($counts, JSON_PRETTY_PRINT)."\n";
