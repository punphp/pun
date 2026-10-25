<?php

namespace UnsetTransfers;

function explicitAbsence(bool $condition): void
{
    $local = 1;
    unset($local);
    unset($local);
    $local = 2;
    unset($local);
    if ($condition) {
        $maybe = 3;
    }
    unset($maybe);
    unset($maybe);
    unset($_SESSION);
    unset($_SESSION);
}

/** @param array{a: int, 1: int} $array
 * @param 'a'|'missing' $key
 */
function finiteUnion(array $array, string $key): void
{
    unset($array[$key]);
    unset($array[1]);
    unset($array[1]);
}

/** @param array{1: int, a: int} $array */
function generalKeys(array $array, string $string, int $integer): void
{
    unset($array[$string]);
    unset($array['missing']);
    unset($array[$integer]);
    unset($array['missing']);
}

/** @template T of array{a: int}
 * @param T $array
 */
function template(array $array): void
{
    unset($array['a']);
    unset($array['missing']);
}

/** @param array{z: mixed, a: int, ...<string, int>} $open
 * @param array{z: mixed, a: int} $unrelated
 */
function incoming(array $open, array $unrelated): void
{
    unset($open[10]);
    \PHPStan\dumpType($unrelated);
    unset($unrelated['missing']);
}

/** @param callable&array $callable */
function invalidatedCallable(array $callable): void
{
    unset($callable[2]);
    $callable = ['strlen', 'invoke'];
    unset($callable['missing']);
}

/** @param array{z: mixed, a: int} $array */
function invalidatedShape(array $array): void
{
    unset($array['a']);
    $array = ['other' => 3];
    unset($array['missing']);
}

/** @param callable&array $callable */
function callableMetadataBetweenTargets(array $callable): void
{
    unset($callable[0], $callable['other']);
}

/** @param callable&array $callable */
function callableMetadataBetweenStatements(array $callable): void
{
    unset($callable[0]);
    unset($callable['other']);
}

/** @param array{z: mixed, a: int} $array */
function indexReassignsReceiver(array $array): void
{
    unset($array[$array = ['a' => 1, 'z' => 2]]);
}

/** @param array{z: mixed, a: int} $array */
function actualNarrowingWins(array $array): void
{
    assert($array['a'] === 7);
    unset($array['missing']);
    unset($array['a']);
    unset($array['a']);
}
