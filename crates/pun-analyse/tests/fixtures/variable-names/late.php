<?php

namespace VariableNameLateTypes;

/**
 * @template T of int|string
 * @param T $value
 * @param (T is int ? 'intBranch' : 'stringBranch') $name
 */
function conditionalName($value, $name): void
{
    compact($name);
    echo $$name;
    $copy = $name;
    compact($copy);
}

/**
 * @template T of int|string
 * @param T $value
 * @param (T is int ? array{'intArray'} : array{'stringArray'}) $names
 */
function conditionalArray($value, $names): void
{
    compact($names);
    $copy = $names;
    compact($copy);
}

/**
 * @template T of int|string
 * @param T $value
 * @param (T is int ? 'notCertain' : string) $name
 * @param (T is int ? array{'notCertainArray'} : array<string>) $names
 */
function conditionalGeneralControls($value, $name, $names): void
{
    compact($name, $names);
    echo $$name;
}

/** @param 'literal'|int $name */
function strictUnion($name): void
{
    compact($name);
    echo $$name;
}

/** @param array{'literalArray'}|array<string> $names */
function strictArrayUnion($names): void
{
    compact($names);
}
