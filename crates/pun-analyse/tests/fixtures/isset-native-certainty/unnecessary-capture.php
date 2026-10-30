<?php

// php unnecessary-capture.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$capture = str_replace(
    ['native-certainty-matrix.php', 'native-certainty-phpstan.json', '    phpVersion: 80500\n'],
    ['certainty-unnecessary.php', 'certainty-unnecessary-phpstan.json',
        '    phpVersion: 80500\n    featureToggles:\n        unnecessaryNullCoalesce: true\n'],
    file_get_contents(__DIR__ . '/native-certainty-capture.php'),
);
eval(substr($capture, strlen('<?php')));
