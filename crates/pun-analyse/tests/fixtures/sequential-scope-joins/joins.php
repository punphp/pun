<?php declare(strict_types=1);

namespace SequentialScopeJoins;

function independent(?int $a, ?string $b, ?bool $c): array
{
    $result = [];
    if ($a !== null) {
        $result['a'] = $a;
    }
    \PHPStan\dumpType($result);
    if ($b !== null) {
        $result['b'] = $b;
    }
    \PHPStan\dumpType($result);
    if ($c !== null) {
        $result['c'] = $c;
    }
    \PHPStan\dumpType($result);
    return $result;
}

function anchored(bool $a, bool $b, bool $c): array
{
    $result = ['a' => 1];
    if ($a) {
        $result['b'] = 'a';
    }
    if ($b) {
        $result['c'] = 'b';
    }
    if ($c) {
        $result['d'] = true;
    }
    \PHPStan\dumpType($result);
    return $result;
}

function tagged(bool $choose, bool $append): array
{
    if ($choose) {
        $result = ['kind' => 'left', 'payload' => 1];
    } else {
        $result = ['kind' => 'right', 'payload' => 2];
    }
    if ($append) {
        $result['z_optional'] = 3;
    }
    \PHPStan\dumpType($result);
    if ($result['kind'] === 'left') {
        \PHPStan\dumpType($result);
    }
    return $result;
}

function requiredEmptyAlternative(bool $choose, bool $append): array
{
    if ($choose) {
        $result = [];
    } else {
        $result = ['a' => 1, 'b' => 2];
    }
    \PHPStan\dumpType($result);
    if ($append) {
        $result['c'] = 3;
    }
    \PHPStan\dumpType($result);
    return $result;
}

/**
 * @template T of list<never>
 * @param T $empty
 */
function templateEmpty(bool $first, array $empty): array
{
    if ($first) {
        $result = $empty;
    } else {
        $result = [];
    }
    \PHPStan\dumpType($result);
    return $result;
}
