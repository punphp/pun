<?php

$declarations = dirname(__DIR__, 4).'/pun-lint/tests/phpunit-stubs/';
spl_autoload_register(static function (string $class) use ($declarations): void {
    if (!str_starts_with($class, 'PHPUnit\\')) {
        return;
    }
    $file = $declarations.str_replace('\\', '/', substr($class, 8)).'.php';
    if (is_file($file)) {
        require_once $file;
    }
});
