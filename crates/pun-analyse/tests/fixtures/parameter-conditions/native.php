<?php declare(strict_types=1);

namespace ParameterNative;

/** @param ($flag is true ? int : string) $value */
function values(bool $flag, int|string $value): void
{
    $value; // observe
    if ($flag) {
        $value; // observe
    } else {
        $value; // observe
    }
    $value; // observe
    $closure = function () use ($flag, $value): void {
        if ($flag) {
            $value; // observe
        }
    };
    $arrow = fn () => $flag ? $value /* observe */ : $value /* observe */;
    $value; // observe
}
