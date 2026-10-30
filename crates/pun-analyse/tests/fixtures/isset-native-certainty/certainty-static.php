<?php

namespace IssetCertaintyStatic;

/** @template T */
class Holder
{
    public static int $value = 1;
}

/** @param Holder<int> $holder */
function fetch(Holder $holder): bool
{
    return isset($holder::$value);
}

/** @param Holder<int> $holder */
function coalesce(Holder $holder): int
{
    return $holder::$value ?? 0;
}
