<?php

namespace ClosureFromCallableForms;

function identity(string $value): string { return $value; }

final class Transformer {
    public static function transformStatic(string $value): string { return $value; }
    public function transform(string $value): string { return $value; }
    public function __invoke(string $value): string { return $value; }
}

function forms(Transformer $transformer): void {
    \PHPStan\dumpType(\Closure::fromCallable('ClosureFromCallableForms\\identity'));
    \PHPStan\dumpType(\Closure::fromCallable('\\ClosureFromCallableForms\\identity'));
    \PHPStan\dumpType(\Closure::fromCallable('ClosureFromCallableForms\\Transformer::transformStatic'));
    \PHPStan\dumpType(\Closure::fromCallable([Transformer::class, 'transformStatic']));
    \PHPStan\dumpType(\Closure::fromCallable([$transformer, 'transform']));
    \PHPStan\dumpType(\Closure::fromCallable($transformer));
    \PHPStan\dumpType(\Closure::fromCallable(fn(string $value): string => $value));
    \PHPStan\dumpType(\Closure::fromCallable(identity(...)));
    \PHPStan\dumpType(\Closure::fromCallable(callback: [\DOMXPath::class, 'quote']));
    \PHPStan\dumpType(\Closure::fromCallable('strlen'));
    $assigned = \Closure::fromCallable(callback: [$transformer, 'transform']);
    \PHPStan\dumpType($assigned);
    \PHPStan\dumpType($assigned('value'));
    \PHPStan\dumpType((\Closure::fromCallable('ClosureFromCallableForms\\identity'))('value'));
}

/** @param callable(string): string $callback */
function typedCallable(callable $callback): void {
    \PHPStan\dumpType(\Closure::fromCallable($callback));
    $assigned = \Closure::fromCallable($callback);
    \PHPStan\dumpType($assigned('value'));
}

/** @param \Closure(string): string $callback */
function typedClosure(\Closure $callback): void {
    \PHPStan\dumpType(\Closure::fromCallable($callback));
    $assigned = \Closure::fromCallable($callback);
    \PHPStan\dumpType($assigned('value'));
}
