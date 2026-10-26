<?php declare(strict_types = 1);

namespace ArrayMutationStates;

/** @param array<string, int> $values */
function multipleIterations(array $values): void
{
    $values['remove'] = 1;
    foreach (['absent', 'remove'] as $name) {
        if (isset($values[$name])) {
            unset($values[$name]);
        }
    }
    \PHPStan\dumpType($values);
    \PHPStan\dumpType(count($values));
    if (count($values) > 0) {
        echo 'possibly nonempty';
    }
}

/**
 * @param array<string, int> $values
 * @param list<'remove'|'absent'> $names
 */
function zeroOrMultipleIterations(array $values, array $names): void
{
    $values['remove'] = 1;
    foreach ($names as $name) {
        if (isset($values[$name])) {
            unset($values[$name]);
        }
    }
    \PHPStan\dumpType($values);
    \PHPStan\dumpType(count($values));
}

/** @param array<string, int> $values */
function singleLiteralIteration(array $values): void
{
    $values['remove'] = 1;
    $values['keep'] = 2;
    foreach (['remove'] as $name) {
        if (isset($values[$name])) {
            unset($values[$name]);
        }
    }
    \PHPStan\dumpType($values);
    \PHPStan\dumpType(count($values));
}

/** @param array<string, int> $values */
function noMutation(array $values): void
{
    $values['keep'] = 2;
    foreach (['absent', 'remove'] as $name) {
        echo $name;
    }
    \PHPStan\dumpType($values);
}
