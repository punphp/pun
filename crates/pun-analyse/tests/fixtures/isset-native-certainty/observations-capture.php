<?php

// php observations-capture.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$capture = file_get_contents(__DIR__ . '/native-certainty-capture.php');
$capture = str_replace(
    ['foreach ([1, 4] as $level)', "['on' => true, 'off' => false]", "['default' => null, 'on' => true, 'off' => false]",
        'native-certainty-matrix.php', 'native-certainty-phpstan.json'],
    ['foreach ([0] as $level)', "['on' => true]", "['default' => null]",
        'native-observations.php', 'native-observations-phpstan.json'],
    $capture,
);
eval(substr($capture, strlen('<?php')));
