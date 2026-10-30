<?php

namespace CallableArrayDescriptionControls;

class Receiver
{
    public function invoke(): void {}
}

function literal(Receiver $receiver): void
{
    $callback = [$receiver, 'invoke'];
    \PHPStan\dumpType($callback);
    $reverse = [1 => 'invoke', 0 => $receiver];
    \PHPStan\dumpType($reverse);
    $ordinary = [1 => 'ordinary', 0 => 42];
    \PHPStan\dumpType($ordinary);
}

/** @param array{0: object, 1: non-empty-string} $ordinary */
function ordinary(array $ordinary): bool
{
    \PHPStan\dumpType($ordinary);
    return isset($ordinary[0]);
}

/** @param array{0: object, 1: non-empty-string}|callable $value */
function unrelated(array|callable $value): void
{
    if (!is_callable($value)) {
        \PHPStan\dumpType($value);
    }
}

/** @param array{0: object, 1: non-empty-string} $value */
function narrowedOrdinary(array $value): void
{
    if ($value[1] === 'invoke') {
        \PHPStan\dumpType($value);
    }
    unset($value[0]);
    \PHPStan\dumpType($value);
}

/** @param array{0: object, 1: non-empty-string}&callable $value */
function literalReplacement(array $value): void
{
    $value = [new Receiver(), 'missing'];
    \PHPStan\dumpType($value);
}
