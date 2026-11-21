<?php

declare(strict_types=1);

$level = 1;
$ignore = [];

if (interface_exists(PHPStan\Analyser\IgnoreErrorExtension::class)) {
    $level = 4;
}
if (class_exists(\PHPStan\Analyser\Analyser::class)) {
    $ignore[] = '#^phpstan class$#';
}
if (class_exists(PHPStan\Analyser\IgnoreErrorExtension::class)) {
    $ignore[] = '#^phpstan interface as a class$#';
}
if (interface_exists('phpstan\analyser\analyser')) {
    $ignore[] = '#^phpstan class as an interface$#';
}
if (class_exists(PHPStan\Analyser\NoSuchClass::class)) {
    $ignore[] = '#^missing phpstan class$#';
}
if (class_exists(PhpParser\Node\Expr\Variable::class)) {
    $ignore[] = '#^php-parser class$#';
}
if (interface_exists(Countable::class)) {
    $ignore[] = '#^php interface$#';
}

return ['parameters' => ['level' => $level, 'ignoreErrors' => $ignore, 'reportUnmatchedIgnoredErrors' => false]];
