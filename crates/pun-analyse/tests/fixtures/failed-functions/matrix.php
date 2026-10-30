<?php
namespace FailedNamedFunctions;

use function AbsentFunctionLibrary\missing as imported_missing;

function known(int $value): int { return $value; }

function observations(string $text): void {
    \PHPStan\dumpType(missing());
    $failed = missing_with_arguments($evaluated = 3);
    \PHPStan\dumpType($failed);
    \PHPStan\dumpType($evaluated);
    \PHPStan\dumpType(\pun_missing_global_function());
    \PHPStan\dumpType(imported_missing());
    \PHPStan\dumpType(known(7));
    \PHPStan\dumpType(strlen($text));
    \PHPStan\dumpType(\str_repeat($text, 2));
    $closure = static fn (): string => 'closure';
    \PHPStan\dumpType($closure());
}

function dynamic_condition(int $subject): void {
    $call = 'FailedNamedFunctions\\missing_dynamic_target';
    $result = match ($subject) { $call() => 1, default => 2 };
}
