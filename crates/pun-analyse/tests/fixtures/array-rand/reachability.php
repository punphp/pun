<?php

namespace ArrayRand;

use function PHPStan\dumpType;

function stop(): never
{
    throw new \RuntimeException();
}

/** @return never */
function docStop()
{
    throw new \RuntimeException();
}

/** @return ($num is 0 ? never : int) */
function conditionalStop(int $num): int
{
    if ($num === 0) {
        throw new \RuntimeException();
    }
    return $num;
}

function emptyNested(): void
{
    $key = array_rand(['only' => array_rand([])]);
    dumpType($key);
    dumpType('reachable');
}

function explicitNever(): void
{
    stop();
    echo 'unreachable';
}

function phpDocNever(): void
{
    docStop();
    echo 'unreachable';
}

function conditionalNever(): void
{
    conditionalStop(0);
    echo 'unreachable';
}

function nestedExplicitNever(): void
{
    array_rand(['only' => stop()]);
    echo 'unreachable';
}

function shortCircuit(bool $flag): void
{
    $flag && stop();
    dumpType('reachable');
}

function unevaluatedClosure(): void
{
    $closure = fn () => array_rand(['only' => stop()]);
    dumpType($closure);
    dumpType('reachable');
    echo 'reachable';
}
