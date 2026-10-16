<?php

// Render docs/presets/ranking.md from rank.php output + exported fixtures.
//   php scripts/ranking.php /tmp/rank.json > docs/presets/ranking.md

$root = dirname(__DIR__);
$counts = json_decode(file_get_contents($argv[1]), true);
$effective = json_decode(file_get_contents("$root/docs/presets/pinkary-effective.json"), true);
$targets = array_keys(reset($counts));

$rows = [];
foreach ($effective['fixers'] as $f) {
    $fx = @json_decode(file_get_contents("$root/crates/pun-harness/fixtures/{$f['name']}.json"), true);
    $fixCases = 0;
    foreach ($fx['tests'] ?? [] as $t) {
        $fixCases += str_starts_with($t['method'], 'testFix') ? count($t['cases']) : 0;
    }
    $c = $counts[$f['name']] ?? [];
    $rows[] = ['name' => $f['name'], 'priority' => $f['priority'], 'c' => $c, 'total' => array_sum($c),
        'fix' => $fixCases, 'all' => $fx['case_count'] ?? 0];
}
usort($rows, fn ($a, $b) => [$b['total'], $a['fix'], $a['name']] <=> [$a['total'], $b['fix'], $b['name']]);

echo "# Fixer impact ranking (Pinkary effective rules)\n\n";
echo "Files changed by each fixer **alone** (options from `pinkary-effective.json`), via `scripts/rank.php`.\n";
echo "`pinkary-dirty` = Pinkary through `scripts/perturb.php`. Sorted by total changed files desc, then\n";
echo "fewest fix cases. `fix cases` = cases feeding `testFix*` methods; `all cases` includes other providers.\n";
echo "`Pint/laravel_blade` (Prettier, Blade only) is not ranked.\n\n";
echo '| # | fixer | priority | '.implode(' | ', $targets)." | total | fix cases | all cases |\n";
echo '|---|---|---|'.str_repeat('---|', count($targets))."---|---|---|\n";
foreach ($rows as $i => $r) {
    $cells = array_map(fn ($t) => $r['c'][$t] ?? '-', $targets);
    printf("| %d | %s | %d | %s | %d | %d | %d |\n", $i + 1, $r['name'], $r['priority'], implode(' | ', $cells), $r['total'], $r['fix'], $r['all']);
}
