<?php

namespace ClosureFromCallableUnions;

function stringIdentity(string $value): string { return $value; }

final class Transformer {
    public function transform(string $value): string { return $value; }
}

function resolved(bool $choose): void {
    $closure = \Closure::fromCallable($choose ? 'ClosureFromCallableUnions\\stringIdentity' : 'strlen');
    \PHPStan\dumpType($closure);
    \PHPStan\dumpType($closure('value'));
}

/** @param callable-string $unknown */
function unknownString(bool $choose, Transformer $transformer, string $unknown): void {
    $closure = \Closure::fromCallable($choose ? [$transformer, 'transform'] : $unknown);
    \PHPStan\dumpType($closure('value'));
    \PHPStan\dumpType($closure(123));
    $unresolved = \Closure::fromCallable($unknown);
    \PHPStan\dumpType($unresolved('value'));
}

function unknownClosure(bool $choose, Transformer $transformer, \Closure $unknown): void {
    $closure = \Closure::fromCallable($choose ? [$transformer, 'transform'] : $unknown);
    \PHPStan\dumpType($closure('value'));
    \PHPStan\dumpType($closure(123));
    $unresolved = \Closure::fromCallable($unknown);
    \PHPStan\dumpType($unresolved('value'));
}

function generalString(bool $choose, Transformer $transformer, string $unknown): void {
    $closure = \Closure::fromCallable($choose ? [$transformer, 'transform'] : $unknown);
    \PHPStan\dumpType($closure('value'));
    \PHPStan\dumpType($closure(123));
    $unresolved = \Closure::fromCallable($unknown);
    \PHPStan\dumpType($unresolved('value'));
}
