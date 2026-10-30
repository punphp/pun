<?php

namespace ParameterNativeClampChannels;

class Box {}
class Child extends Box {}

/**
 * @template T of Box
 * @param T $value
 * @return T
 */
function identity(Box $value): Box
{
    return $value;
}

/** @param \Closure(int): string $callback */
function invoke(\Closure $callback): string
{
    return $callback(1);
}

function requiresChild(Child $value): void {}
function controls(Child $child): void
{
    requiresChild(identity($child));
    invoke(static fn (int $value): string => (string) $value);
}
