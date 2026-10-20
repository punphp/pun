<?php

namespace CoverageIncDec;

/**
 * @template T
 * @param T $t
 */
function template($t, bool $b, ?int $n, string $s): void
{
    $t++;
    $b++;
    $b--;
    $n--;
    $s--;
    $s++;
}

/**
 * @template T
 * @param T $a
 * @return T
 */
function same($a)
{
    return $a;
}

/**
 * @template T
 * @template U
 * @param T $a
 * @return U
 */
function other($a)
{
    return $a;
}
