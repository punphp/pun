<?php

// Writes crates/pun-lint/data/tentative-methods.txt: PHP's built-in methods whose return type is only tentative
// (ReflectionMethod::hasTentativeReturnType()), as `class::method`, lowercase, one a line, from the running PHP 8.5.
// PHPStan's native type of a call to one is `mixed` (Rector's getNativeType uses it unless the receiver is a built-in
// class). The PHP that runs this decides which extensions' classes are listed.
//
//   php scripts/export-tentative-methods.php

$out = [];
foreach (array_merge(get_declared_classes(), get_declared_interfaces()) as $class) {
    $reflection = new ReflectionClass($class);
    if (! $reflection->isInternal()) {
        continue;
    }
    foreach ($reflection->getMethods() as $method) {
        if ($method->getDeclaringClass()->getName() === $reflection->getName() && $method->hasTentativeReturnType()) {
            $out[] = strtolower($reflection->getName().'::'.$method->getName());
        }
    }
}
$out = array_unique($out);
sort($out);
file_put_contents(dirname(__DIR__).'/crates/pun-lint/data/tentative-methods.txt', implode("\n", $out)."\n");
