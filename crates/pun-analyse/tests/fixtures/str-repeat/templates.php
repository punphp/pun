<?php
namespace RepeatTemplates;
use function PHPStan\dumpType;

/** @template T of 'a'
 * @template N of 2
 * @param T $input
 * @param N $count
 */
function constants(string $input, int $count): void
{
    dumpType(str_repeat($input, $count));
    dumpType(str_repeat($input, 100));
    dumpType(987);
}

/** @template T of non-empty-string
 * @template N of positive-int
 * @param T $input
 * @param N $count
 */
function bounds(string $input, int $count): void
{
    dumpType(str_repeat($input, $count));
    dumpType(str_repeat($input, 1));
    dumpType(str_repeat('x', $count));
    dumpType(987);
}

/** @param class-string $class */
function classes(string $class): void
{
    dumpType(str_repeat($class, 1));
    dumpType(str_repeat(\stdClass::class, 2));
}

/** @template T of 'a'
 * @template U of T
 * @param U $input
 */
function nested(string $input): void
{
    dumpType(str_repeat($input, 2));
    dumpType(str_repeat($input, 100));
    dumpType(987);
}

/** @param 'a'|int $input */
function domains($input, $mixed): void
{
    dumpType(str_repeat($input, 2));
    dumpType(str_repeat($mixed, 2));
    dumpType(str_repeat(false, 2));
    dumpType(str_repeat('x', 1.0));
    dumpType(str_repeat('x', true));
    dumpType(str_repeat(stream_get_contents(STDIN), 2));
    dumpType(987);
}

/** @template T of \stdClass::class
 * @param T $input
 */
function classLiteral(string $input): void
{
    dumpType(str_repeat($input, 2));
    dumpType(str_repeat($input, 100));
}

/** @template N of 2
 * @template M of N
 * @param M $count
 */
function nestedCount(int $count): void
{
    dumpType(str_repeat('a', $count));
    dumpType(987);
}
