<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

/**
 * @template T
 * @param T $value
 * @return T
 * @phpstan-pure
 */
function pureIdentity(mixed $value): mixed { return $value; }

/**
 * @template T
 * @param T $value
 * @return T
 * @phpstan-impure
 */
function impureIdentity(mixed $value): mixed { echo ''; return $value; }

function purity(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $pure = pureIdentity($key);
    $impure = impureIdentity($key);
    if ($method === 'first') {
        \PHPStan\dumpType($pure);
        \PHPStan\dumpType($impure);
    } else {
        \PHPStan\dumpType($pure);
        \PHPStan\dumpType($impure);
    }
    return true;
}

function nestedWrite(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $counter = 0;
    $copy = [$key, $counter++];
    if ($method === 'first') {
        \PHPStan\dumpType($copy[0]);
        \PHPStan\dumpType($counter);
    } else {
        \PHPStan\dumpType($copy[0]);
        \PHPStan\dumpType($counter);
    }
    return true;
}
