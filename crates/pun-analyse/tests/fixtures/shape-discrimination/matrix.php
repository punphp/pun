<?php

namespace ShapeDiscrimination;

function twoDifferentFields(bool $branch): array
{
    $pair = $branch ? ['m' => 'foo', 'v' => 1] : ['m' => 'bar', 'v' => 's'];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function twoDifferentFieldsReverse(bool $branch): array
{
    $pair = $branch ? ['m' => 'bar', 'v' => 's'] : ['m' => 'foo', 'v' => 1];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function oneDifferentField(bool $branch): array
{
    $pair = $branch ? ['m' => 'foo', 'v' => 1] : ['m' => 'bar', 'v' => 1];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function oneDifferentFieldReverse(bool $branch): array
{
    $pair = $branch ? ['m' => 'bar', 'v' => 1] : ['m' => 'foo', 'v' => 1];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function singletonShapes(bool $branch): array
{
    $pair = $branch ? ['m' => 'foo'] : ['m' => 'bar'];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['m']);
        \PHPStan\dumpType([$pair['m']]);
    } else {
        \PHPStan\dumpType($pair['m']);
        \PHPStan\dumpType([$pair['m']]);
    }
    return $pair;
}

function singletonShapesReverse(bool $branch): array
{
    $pair = $branch ? ['m' => 'bar'] : ['m' => 'foo'];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['m']);
        \PHPStan\dumpType([$pair['m']]);
    } else {
        \PHPStan\dumpType($pair['m']);
        \PHPStan\dumpType([$pair['m']]);
    }
    return $pair;
}

function overlappingValues(bool $branch, bool $wide): array
{
    $value = $wide ? 1 : 's';
    $pair = $branch ? ['m' => 'foo', 'v' => 1] : ['m' => 'bar', 'v' => $value];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function overlappingValuesReverse(bool $branch, bool $wide): array
{
    $value = $wide ? 1 : 's';
    $pair = $branch ? ['m' => 'bar', 'v' => $value] : ['m' => 'foo', 'v' => 1];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function asymmetricKeys(bool $branch): array
{
    $pair = $branch ? ['extra' => true, 'm' => 'foo', 'v' => 1] : ['m' => 'bar', 'v' => 's'];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function asymmetricKeysReverse(bool $branch): array
{
    $pair = $branch ? ['m' => 'bar', 'v' => 's'] : ['extra' => true, 'm' => 'foo', 'v' => 1];
    \PHPStan\dumpType($pair);
    if ($pair['m'] === 'foo') {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    } else {
        \PHPStan\dumpType($pair['v']);
        \PHPStan\dumpType([$pair['v']]);
    }
    return $pair;
}

function equalShapes(bool $branch): array
{
    $pair = $branch ? ['m' => 'foo', 'v' => 1] : ['m' => 'foo', 'v' => 1];
    \PHPStan\dumpType($pair);
    \PHPStan\dumpType($pair['v']);
    return $pair;
}

function ordinaryUnion(bool $branch): int|string
{
    $value = $branch ? 1 : 's';
    \PHPStan\dumpType($value);
    if ($value === 1) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    return $value;
}

function ordinaryUnionReverse(bool $branch): int|string
{
    $value = $branch ? 's' : 1;
    \PHPStan\dumpType($value);
    if ($value === 1) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    return $value;
}
