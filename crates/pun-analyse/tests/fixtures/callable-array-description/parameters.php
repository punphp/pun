<?php

namespace CallableArrayDescription;

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function documented(array $callback): bool
{
    return isset($callback[0]);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function untyped($callback): bool
{
    return isset($callback[0]);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function copies(array $callback): void
{
    \PHPStan\dumpType($callback);
    $copy = $callback;
    \PHPStan\dumpType($copy);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function narrowed(array $callback): void
{
    if ($callback[1] === 'invoke') {
        \PHPStan\dumpType($callback);
        \PHPStan\dumpType($callback[1]);
    }
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function mutations(array $callback): void
{
    $copy = $callback;
    unset($callback[1]);
    \PHPStan\dumpType($callback);
    \PHPStan\dumpType($copy);
    $copy[2] = 7;
    \PHPStan\dumpType($copy);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function replaced(array $callback): void
{
    $copy = $callback;
    $callback = [42, 'not callable'];
    \PHPStan\dumpType($callback);
    \PHPStan\dumpType($copy);
}

/** @param array{1: non-empty-string, 0: object}&callable $callback */
function reversed(array $callback): void
{
    \PHPStan\dumpType($callback);
    if ($callback[1] === 'invoke') {
        \PHPStan\dumpType($callback);
    }
}
