<?php
// Usage: php scripts/export-rule-sets.php .oracle/pint/vendor/autoload.php > crates/pun-config/presets/sets.json
// Prints every PHP-CS-Fixer rule set's getRules() (nested sets unresolved) for pun-config. The @auto sets read the
// project's composer.json, so they are left out and `pun migrate` names them as unsupported.
require $argv[1];
error_reporting(E_ALL & ~E_DEPRECATED & ~E_USER_DEPRECATED);
$out = [];
foreach (PhpCsFixer\RuleSet\RuleSets::getSetDefinitions() as $name => $set) {
    if (str_starts_with($name, '@auto')) continue;
    $out[$name] = $set->getRules();
}
ksort($out);
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES), "\n";
