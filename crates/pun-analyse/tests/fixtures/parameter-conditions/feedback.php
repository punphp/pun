<?php declare(strict_types=1);

namespace ParameterFeedback;

/** @param 'left'|'right' $mode */
function local(string $mode): void
{
    $mode; // observe before-local
    $reader = function () use (&$mode): void {
        $mode; // observe inside-local
    };
    $mode; // observe after-local
    $copy = fn () => $mode /* observe arrow-local */;
    $mode = 'later';
    $mode; // observe written-local
}

/** @param 'yes'|'no' $mode */
function narrowed(string $mode): void
{
    $mode; // observe before-narrowed
    $reader = function () use (&$mode): void {
        if ($mode !== 'yes') {
            return;
        }
        $mode; // observe inside-narrowed
    };
    $mode; // observe after-narrowed
}

/** @param 'cold'|'hot' $mode */
function opaqueBody(string $mode): void
{
    $mode; // observe before-opaque
    $reader = function () use (&$mode): void {
        [$mode];
        $mode; // observe inside-opaque
    };
    $mode; // observe after-opaque
}

/** @param 'up'|'down' $mode */
function escaped(string $mode): \Closure
{
    $mode; // observe before-escape
    $reader = function () use (&$mode): string {
        return $mode /* observe inside-escape */;
    };
    $mode; // observe after-escape
    return $reader;
}

/**
 * @param 1|3 $number
 * @param array{tag: 'old', count: 2} $shape
 */
function separate(int $number, array $shape): void
{
    $number; // observe before-number
    $shape; // observe before-shape
    $reader = function () use (&$number, &$shape): array {
        $number; // observe inside-number
        return $shape /* observe inside-shape */;
    };
    $number; // observe after-number
    $shape; // observe after-shape
}

/** @param 'initial' $value */
function byValue(string $value): void
{
    $value; // observe before-value
    $reader = function () use ($value): string {
        return $value /* observe inside-value */;
    };
    $value; // observe after-value
}
