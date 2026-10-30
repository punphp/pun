<?php

namespace IssetCertaintyUnnecessary;

/** @param array{required: ?int} $shape */
function required(array $shape): ?int
{
    return $shape['required'] ?? null;
}

/** @param array{optional?: ?int} $shape */
function optional(array $shape): ?int
{
    return $shape['optional'] ?? null;
}

/** @param array{nested: array{leaf: ?int}} $shape */
function nested(array $shape): ?int
{
    return $shape['nested']['leaf'] ?? null;
}
