<?php

namespace PunFunctionCallablePartial;

function arguments(int $first, int $second): int { return $first + $second; }

function controls(): void
{
    arguments(?, 1);
    arguments(1, ...);
    $integer = 1;
    $integer(?, 1);
    $integer(1, ...);
}
