<?php declare(strict_types = 1);

namespace ArrayMutationStates;

/** @param array<string, int> $values */
function literalListKeys(array $values): void
{
    $values['remove'] = 1;
    foreach (['absent', 'remove'] as $index => $name) {
        unset($values[$name]);
        echo $index;
    }
    \PHPStan\dumpType($values);
}

/** @param array<string, int> $values */
function emptyIteration(array $values): void
{
    $values['keep'] = 1;
    foreach ([] as $name) {
        unset($values[$name]);
    }
    \PHPStan\dumpType($values);
    echo 'empty';
}

/** @param array<string, int> $values */
function unrollLimit(array $values): void
{
    $values['remove'] = 1;
    foreach (['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'remove'] as $name) {
        unset($values[$name]);
    }
    \PHPStan\dumpType($values);
    echo 'limit';
}

/**
 * @param array<string, int> $values
 * @param array{0?: 'remove'} $names
 */
function optionalIteration(array $values, array $names): void
{
    $values['remove'] = 1;
    foreach ($names as $name) {
        unset($values[$name]);
    }
    \PHPStan\dumpType($values);
    echo 'optional';
}

/** @param array<string, int> $values */
function earlyBreak(array $values): void
{
    $values['remove'] = 1;
    foreach (['absent', 'remove'] as $name) {
        unset($values[$name]);
        break;
    }
    \PHPStan\dumpType($values);
    echo 'break';
}

/** @param array<string, int> $values */
function destructuredTarget(array $values): void
{
    $values['remove'] = 1;
    $values['keep'] = 2;
    foreach ([['remove']] as [$name]) {
        unset($values[$name]);
    }
    \PHPStan\dumpType($values);
    echo 'destructure';
}

/** @param array<string, int> $values */
function arrayKeyTarget(array $values): void
{
    $values['remove'] = 1;
    $values['keep'] = 2;
    $storage = [];
    foreach (['remove'] as $storage['key'] => $name) {
        unset($values[$name]);
    }
    \PHPStan\dumpType($values);
    echo 'target';
}

/** @param array<string, int> $values */
function byReferenceIteration(array $values): void
{
    $values['remove'] = 1;
    $names = ['absent', 'remove'];
    foreach ($names as &$name) {
        unset($values[$name]);
    }
    \PHPStan\dumpType($values);
    echo 'reference';
}
