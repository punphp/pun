<?php declare(strict_types = 1);

namespace ArrayMutationStates;

/** @param array<string, int> $values */
function literalUnsets(array $values): void
{
    $values['remove'] = 1;
    $values['keep'] = 2;
    unset($values['remove']);
    \PHPStan\dumpType($values);
    unset($values['absent']);
    \PHPStan\dumpType($values);
    unset($values['keep']);
    \PHPStan\dumpType($values);
    echo 'unset';
}

/** @param array<string, int> $values */
function generalUnset(array $values, string $name): void
{
    $values['keep'] = 2;
    unset($values[$name]);
    \PHPStan\dumpType($values);
    echo 'unset';
}

/**
 * @param array<string, int> $values
 * @param 'remove'|'absent' $name
 */
function unionUnset(array $values, string $name): void
{
    $values['remove'] = 1;
    $values['keep'] = 2;
    unset($values[$name]);
    \PHPStan\dumpType($values);
    echo 'unset';
}

/** @param array<string, int> $values */
function guardedUnionUnset(array $values): void
{
    $values['remove'] = 1;
    $values['keep'] = 2;
    foreach (['absent', 'remove'] as $name) {
        if (isset($values[$name])) {
            unset($values[$name]);
        }
    }
    \PHPStan\dumpType($values);
    echo 'unset';
}
