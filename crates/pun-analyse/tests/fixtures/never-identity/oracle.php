<?php

declare(strict_types=1);

// php oracle.php /absolute/path/to/phpstan/bootstrap.php
require $argv[1];

use PHPStan\Type\IntegerType;
use PHPStan\Type\NeverType;
use PHPStan\Type\NonAcceptingNeverType;
use PHPStan\Type\Type;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

$types = [
    'implicit' => new NeverType(),
    'explicit' => new NeverType(true),
    'nonaccepting' => new NonAcceptingNeverType(),
    'int' => new IntegerType(),
];
$record = static fn (Type $type): array => [
    'description' => $type->describe(VerbosityLevel::precise()),
    'kind' => $type instanceof NonAcceptingNeverType ? 'nonaccepting'
        : ($type instanceof NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
];
$out = [];
foreach ($types as $name => $type) {
    $out['types'][$name] = $record($type);
    foreach ($types as $otherName => $other) {
        $out['unions'][$name.'|'.$otherName] = $record(TypeCombinator::union($type, $other));
        $out['intersections'][$name.'&'.$otherName] = $record(TypeCombinator::intersect($type, $other));
    }
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES), "\n";
