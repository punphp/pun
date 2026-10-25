<?php

namespace PunFunctionCallable85;

function arguments(int $first, int $second): int { return $first + $second; }

function controls(): void
{
    $integer = 1;
    'text' |> strlen(...);
    'text' |> nonexistent(...);
    $integer(...);
}
