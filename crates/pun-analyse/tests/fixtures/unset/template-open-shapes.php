<?php

namespace UnsetTemplateOpenShapes;

/** @template T of array{a: int, ...<int, int>}
 * @param T $array
 */
function literalMutation(array $array): void
{
    $copy = $array;
    unset($copy['a']);
    \PHPStan\dumpType($copy);
    unset($copy[0]);
    \PHPStan\dumpType($array);
}

/** @template T of array{a: int, ...<int, int>}
 * @param T $array
 */
function unmatchedOffsetDoesNotRecreate(array $array): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array['a']);
    \PHPStan\dumpType($array);
    unset($array[0]);
}

/** @template T of array{a: int, b: string, ...<int, bool>}
 * @param T $array
 * @param 'a'|'b' $key
 */
function unionMutation(array $array, string $key): void
{
    unset($array[$key]);
    \PHPStan\dumpType($array);
    unset($array[0]);
}

/** @template T of array{a: int, ...<int, string>}
 * @param T $array
 */
function generalMatchingKey(array $array, string $key): void
{
    unset($array[$key]);
    \PHPStan\dumpType($array);
    unset($array[0]);
}

/** @template T of array<int, string>
 * @param T $array
 */
function refinedGenericMustNotBecomeASealedShape(array $array): void
{
    if (isset($array[5])) {
        unset($array[5]);
        unset($array[1]);
    }
}

/** @param array{a: int, ...<int, string>} $array */
function ordinaryOpenShapeKeepsExtras(array $array): void
{
    unset($array['a']);
    unset($array[0]);
    unset($array[0]);
}

/** @template T of array{a: int, ...<int, string>} */
class ClassTemplate
{
    /** @param T $array */
    public function mutate(array $array): void
    {
        unset($array['a']);
        \PHPStan\dumpType($array);
        unset($array[0]);
    }
}

class MethodTemplate
{
    /** @template T of array{a: int, ...<int, string>}
     * @param T $array
     */
    public function mutate(array $array): void
    {
        unset($array['a']);
        \PHPStan\dumpType($array);
        unset($array[0]);
    }
}

/** @template T of array{a?: int, ...<int, string>}
 * @param T $array
 * @param 'a'|'missing' $finite
 */
function optionalMatchesStillRecreate(array $array, string $finite, string $general): void
{
    $copy = $array;
    unset($copy[$finite]);
    \PHPStan\dumpType($copy);
    unset($array[$general]);
    \PHPStan\dumpType($array);
}

/** @template T of array{a?: int, ...<int, string>}
 * @param T $array
 * @param 'b'|'c' $finite
 */
function disjointUnionDoesNotRecreate(array $array, string $finite): void
{
    unset($array[$finite]);
    \PHPStan\dumpType($array);
    unset($array[0]);
}
